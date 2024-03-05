import fs from "fs";
import _ from "lodash";

let fmbfamily = JSON.parse(fs.readFileSync("FMBFAMILY.json", "utf8"));
let sections = fmbfamily.sections;

let createHeaderMap = (row, headerMap) => {
  Object.entries(row).forEach(([key, value]) => {
    if (value.text) {
      headerMap[key] = _.snakeCase(value.text);
    }
  });
};
let createHeaderMapFromRow = (row, headerMap) => {
  Object.entries(row).forEach(([key, value]) => {
    if (key) {
      headerMap[key] = _.snakeCase(key);
    }
  });
};

let rowWithHeader = (row, headerMap) => {
  let newRow = {};
  Object.entries(row).forEach(([key, value]) => {
    if (value.text) {
      newRow[headerMap[key]] = value.text;
    }
  });
  return newRow;
};

let finalTables = {};
for (let section of sections) {
  let headerPosition = 0;
  finalTables[section.title] = [];
  for (let table of section?.tables || []) {
    // We have different kind of tables and headers there
    let headerMap = {};
    if (table[0].col1) {
      if (table[0].col1?.text == table[1].col1?.text) {
        createHeaderMap(table[1], headerMap);
        headerPosition = 2;
      } else {
        createHeaderMap(table[0], headerMap);
        headerPosition = 1;
      }
    } else {
      createHeaderMapFromRow(table[0], headerMap);
      headerPosition = 0;
    }

    // Collect rows for table
    let finalTableRows = [];

    for (let rowIndex = headerPosition; rowIndex < table.length; rowIndex++) {
      let row = table[rowIndex];
      if (row.col1) {
        let newRow = rowWithHeader(row, headerMap);
        finalTableRows.push(newRow);
      }
    }

    // Take the first column of the first table row and group by it
    let grouped = _.groupBy(finalTableRows, (row) => row[Object.keys(row)[0]]);
    // Now ungroup the "grouped" so that collect all object "value" to single array in the ungrouped object
    let processedRows = Object.entries(grouped).map(([parameterId, items]) => {
      // Initialize a new object with a 'value' array to collect the 'value' properties
      let newItem = {};
      // Dynamically add other properties from the first item in the group to the newItem
      // This assumes the first item represents a typical structure for items in this group
      Object.keys(items[0]).forEach((key) => {
        if (key !== "value") {
          // Skip the 'value' property since it's being handled separately
          newItem[key] = items[0][key];
        }
      });

      // Collect all 'value' properties to a single array
      if (items.length > 1) {
        newItem.value = items.map((item) => item.value);
      }
      // Not an array, so let's check if we have a value
      else if ("value" in items[0]) {
        newItem.value = items[0].value;
      }
      return newItem;
    });

    finalTables[section.title].push(processedRows);
  }
}

// Write finalTables to a file
fs.writeFileSync("finalTables.json", JSON.stringify(finalTables, null, 2));
