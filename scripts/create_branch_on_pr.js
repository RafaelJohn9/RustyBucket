module.exports = async ({ github, context, core }) => {
  const pr = context.payload.pull_request;
  if (!pr) {
    core.info("No pull request found.");
    return;
  }

  const username = pr.user.login;
  const repo = context.repo.repo;
  const owner = context.repo.owner;

  const userBranch = `${username}/ratatui`;
  const baseBranch = "main";

  // Check if the user branch exists
  try {
    await github.rest.git.getRef({
      owner,
      repo,
      ref: `heads/${userBranch}`,
    });
    core.info(`Branch ${userBranch} already exists.`);
  } catch (e) {
    if (e.status === 404) {
      core.info(`Branch ${userBranch} does not exist. Creating branch...`);
      // Get the sha of the base branch (main)
      const baseRef = await github.rest.git.getRef({
        owner,
        repo,
        ref: `heads/${baseBranch}`,
      });
      const baseSha = baseRef.data.object.sha;

      // Create new branch ref
      await github.rest.git.createRef({
        owner,
        repo,
        ref: `refs/heads/${userBranch}`,
        sha: baseSha,
      });
      core.info(`Created branch ${userBranch} from ${baseBranch}.`);
    } else {
      throw e;
    }
  }

  // Comment on the PR asking to change base branch
  const issue_number = pr.number;
  const commentBody = `
Hi @${username}! 🎉

We've created a branch called \`${userBranch}\` for you based on \`${baseBranch}\`.

Please update the **base branch** of this pull request from \`${baseBranch}\` to \`${userBranch}\` to proceed with testing and reviews.

Thanks for contributing! 🚀
`;

  await github.rest.issues.createComment({
    owner,
    repo,
    issue_number,
    body: commentBody,
  });
};
