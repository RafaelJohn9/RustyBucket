# 🧀 Ratatui CLI — The Terminal of Taste™

*"Because even a rat deserves a good command-line UX."*

---

## 🍽️ Launching the Kitchen

```bash
cargo run
```

### 🖥️ Output

```
🎩 Bonjour, Chef! Welcome back to La Ratatouille Terminal of Taste™!
✨ "Anyone can cook… but only the fearless can serve during dinner rush." – Gusteau

What would you like to do?
[1] View Menu
[2] Add Dish to Menu
[3] Take New Order
[4] View All Orders
[5] Advance Order Status
[6] Close Restaurant (Exit)

Enter choice: 
```

---

## 🥦 Command: View Menu

```bash
> 1
```

### Output

```
📋 Tonight’s Menu:
- Ratatouille Supreme — A classic Provençal vegetable medley, artfully layered and oven-roasted. Price: $9.50
- Lightning Linguine — Pasta tossed in a zesty tomato sauce with a shocking kick. Price: $8.50
- Fromage Fantastique — A decadent three-cheese creation with a flair for the dramatic. Price: $11.00

Bon appétit! 🍽️
```

---

## 🧑‍🍳 Command: Add Dish to Menu

```bash
> 2
```

### Prompt

```
Enter dish name: 
> Toast à la Burnt

Enter ingredients (comma-separated): 
> toast, smoke, regret
```

### Output

```
✅ “Toast à la Burnt” added to the menu! We hope no one orders it... again.
```

---

## 📝 Command: Take New Order

```bash
> 3
```

### Prompt

```
Enter table number: 
> 5

Available dishes:
1. Ratatouille Supreme
2. Lightning Linguine
3. Fromage Fantastique
4. Toast à la Burnt

Enter dish number: 
> 4
```

### Output

```
🧾 Table 5 has ordered “Toast à la Burnt”.
😬 Bold choice. We admire their courage.
```

---

## 🗃️ Command: View All Orders

```bash
> 4
```

### Output

```
📦 Current Orders:
[Table 5] Toast à la Burnt – Status: Pending
```

---

## 🔥 Command: Advance Order Status

```bash
> 5
```

### Prompt

```
Enter table number: 
> 5
```

### Output

```
🧪 Advancing order for Table 5...
🍳 Status: Pending → Cooking

Remember, don’t actually burn it this time.
```

*Running again:*

```bash
> 5
> 5
```

### Output

```
🍽️ Status: Cooking → Served
🎉 Order delivered to Table 5! Bon appétit! (we warned them)
```

*Running once more:*

```bash
> 5
```

### Output

```
❌ That order is already served, Chef. You can’t cook nostalgia.
```

---

## ❌ Invalid Command Input

```bash
> banana
```

### Output

```
🐒 Uh... Chef? “banana” is not a valid command. This isn’t a fruit stand.
Try typing a number from the list, like a responsible rodent.
```

---

## 🚪 Command: Close Restaurant

```bash
> 6
```

### Output

```
👋 Au revoir! May your soufflés rise and your bugs be shallow!
💡 Pro tip: Don’t forget to clean the terminal before your next guest.
```

---

### 🔥 Bonus: Panic Mode Easter Egg (if 5+ pending orders)

```bash
> 4
```

### Output

```
⚠️ Chef, we’re at MAXIMUM OVERLOAD!
5+ orders are pending! This is not a drill!
Linguini has fainted. Colette is sharpening knives.
Suggest: “Advance Order Status” or hide in the pantry.
```
