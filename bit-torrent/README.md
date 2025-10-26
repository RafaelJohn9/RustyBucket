# Bit Torrent

Torrents are among the most impactful ideas in distributed systems. Whether it’s retro games, old-school movies that may no longer be available online, or a specific resource you need without extra cost, torrents are often the way to go.

This curiosity led me to explore how BitTorrent works internally: what trackers are, how peers discover each other, and how the protocol supports resilient P2P sharing. I wrote an article and implemented a basic BitTorrent client to deepen my understanding.



The following resources helped me learn the protocol and its components:

- [**Build-Your-Own-X** specifically on BitTorrent](https://github.com/RafaelJohn9/build-your-own-x?tab=readme-ov-file#build-your-own-bittorrent-client)
- [**Mark U Seliasson Article** on Building Bit Torrent in Python](https://markuseliasson.se/article/bittorrent-in-python/)
- [**Seán O'Flynn** - Research on BitTorrent](https://www.seanjoflynn.com/research/bittorrent.html)

---

## Main Objectives

1. Understand the **BitTorrent architecture** — trackers, peers, seeds, and leechers.  
2. Implement **torrent file parsing** (reading `.torrent` metadata).  
3. Build or simulate a **tracker** for peer coordination.  
4. Develop a **peer communication system** for piece exchange.  
5. Implement **piece hashing and verification** using SHA-1.  
6. Handle **download scheduling and piece prioritization**.  
7. Experiment with **upload rate limits** and **peer choking/unchoking logic**.  
8. (Optional) Visualize peer interactions or build a small CLI interface for torrent management.

---

##  Progress Checklist


- [x] Set up project directory and basic file structure
- [x] Research and document BitTorrent protocol basics
- [ ] Implement `.torrent` file parser
- [ ] Implement handshake and peer connection logic
- [ ] Implement piece request and data exchange
- [ ] Add piece hashing and verification
- [ ] Build a simple tracker client (HTTP/UDP)
- [ ] Add logging and metrics for debugging
- [ ] Test peer-to-peer file sharing locally
- [ ] Document architecture and flow
- [ ] Write final summary of insights and learnings

---

## Learning Goals

- Deepen understanding of **distributed systems** and **network programming**.  
- Explore **protocol design**, **message serialization**, and **state synchronization**.  
- Learn about **efficiency trade-offs** in data sharing over unreliable networks.  
- Practice designing software that scales across **multiple peers**.

---

## ⚙️ Setup & Usage (to be filled later)

```bash
# Clone the repository and navigate into BitTorrent project
git clone https://github.com/RafaelJohn9/RustyBucket
cd RustyBucket/bit-torrent

# Build and run
cargo run
````

---

##  Notes

* The focus here is not just on implementation, but on **understanding** each layer of the BitTorrent protocol.
* Use this project to experiment, break things, and gain insight into **how distributed sharing networks sustain scalability and reliability**.
* Future improvements may include DHT (Distributed Hash Table) support, magnet links, and parallel piece downloading.

---

### *One peer at a time, the swarm grows stronger.*
