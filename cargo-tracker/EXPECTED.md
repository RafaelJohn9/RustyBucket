# Cargo Tracker CLI: Expected Behavior

---

## CLI Interface

### 1. **Startup**

- On launch, the CLI displays a welcome message and a menu of available commands.

```
Welcome to Cargo Tracker 1.0!
Type 'help' to see a list of available commands.
> help

add-shipment    - Add a new shipment with tracking ID and destination
add-package     - Add a package to an existing shipment
update-status   - Update the status of a shipment
view-shipment   - View details of a specific shipment
list-shipments  - List all shipments (optionally filter by status)
clear           - Clear the screen
help            - Show available commands
exit            - Exit the Cargo Tracker CLI
```

### 2. **Inputs**

The CLI accepts the following commands (case-insensitive):

```text
> add-shipment

Please enter the Tracking ID: ABC123
Please enter the destination: Nairobi
Shipment Created

Let's add packages. Type 'q' to quit
Enter package #1 description: Electronics
Enter package #2 description: Books
Enter package #3 description: q
2 packages added to shipment 'ABC123'.

> add-package

Please enter the Tracking ID: ABC123
Enter package #1 description: Headphones
Package added to shipment 'ABC123'.

> update-status

Please enter the Tracking ID: ABC123
Enter new status (Pending, InTransit, Delivered, Lost): InTransit
Shipment 'ABC123' status updated to InTransit.

> view-shipment

Please enter the Tracking ID: ABC123
Tracking ID: ABC123
Destination: Nairobi
Status: InTransit
Packages:
    - Electronics
    - Books
    - Headphones

> list-shipments

Tracking ID: ABC123 | Destination: Nairobi | Status: InTransit | Packages: 3

> help

Available commands:
    add-shipment    - Add a new shipment with tracking ID and destination
    add-package     - Add a package to an existing shipment
    update-status   - Update the status of a shipment
    view-shipment   - View details of a specific shipment
    list-shipments  - List all shipments (optionally filter by status)
    clear           - Clear the screen
    help            - Show available commands
    exit            - Exit the Cargo Tracker CLI

> exit

Goodbye!
```

---

## Error Handling

Below are example CLI interactions demonstrating how errors are handled:

```text
> add-shipment

Please enter the Tracking ID: ABC123
Please enter the destination: Nairobi
Shipment Created

> add-shipment

Please enter the Tracking ID: ABC123
Please enter the destination: Mombasa
Error: Shipment with tracking ID 'ABC123' already exists.

> update-status

Please enter the Tracking ID: XYZ999
Error: No shipment found with tracking ID 'XYZ999'.

> update-status

Please enter the Tracking ID: ABC123
Enter new status (Pending, InTransit, Delivered, Lost): Shipped
Error: Invalid status. Valid options are: Pending, InTransit, Delivered, Lost.
```

---

## Notes

- All user input is validated.
- The CLI is menu-driven and guides the user through each step.
- Output is clear and user-friendly.

---

## Advanced Tasks

### Generating Reports

The Cargo Tracker CLI supports generating summary reports for shipments and packages.

#### 1. **Generate Shipment Report**

- Command: `generate-report`
- Prompts the user to select the type of report (e.g., all shipments, by status, by destination).

```text
> generate-report

Select report type:
    1. All Shipments
    2. By Status
    3. By Destination
Enter choice: 2
Enter status (Pending, InTransit, Delivered, Lost): Delivered

--- Shipment Report: Delivered ---
Tracking ID: DEF456 | Destination: Kisumu | Packages: 4
Tracking ID: GHI789 | Destination: Eldoret | Packages: 2
Total Shipments: 2
```

#### 2. **Export Report to File**

- After generating a report, the CLI offers to export the report to a text file.

```text
Would you like to export this report to a file? (y/n): y
Select file format:
    1. Text (.txt)
    2. PDF (.pdf)
    3. Excel (.xlsx)
    4. CSV (.csv)
Enter choice: 3
Enter filename: delivered_report.xlsx
Report saved to 'delivered_report.xlsx'.
```

#### 3. **Error Handling for Reports**

- If no shipments match the criteria, the CLI displays an appropriate message.

```text
> generate-report

Select report type:
    1. All Shipments
    2. By Status
    3. By Destination
Enter choice: 3
Enter destination: Marsabit

No shipments found for destination 'Marsabit'.
```

---
