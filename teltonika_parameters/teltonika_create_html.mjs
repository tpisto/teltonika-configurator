import fs from "fs";
import _ from "lodash";

function createInputField(row) {
  let inputField = "<input ";

  // Analyze input type
  if (_.isArray(row.value)) {
    if (row.value.length == 2 && row.value[0] == "0 – Disable" && row.value[1] == "1 – Enable") {
      inputField += `type="checkbox" `;
    } else {
      inputField += `type="select" `;
      inputField += `options="${row.value.join(",")}" `;
    }

    for (let [key, value] of Object.entries(_.omit(row, "value"))) {
      inputField += `${key}="${value}" `;
    }
  }
  // All non-array values are text or number depending of the parameter type.
  else {
    if (row.parameter_type == "Char") {
      inputField += `type="text" `;
    } else {
      inputField += `type="number" `;
    }
    for (let [key, value] of Object.entries(row)) {
      // Convert " to &quot; in value
      value = value.replace(/"/g, "&quot;");
      // Convert < to &lt; in value
      value = value.replace(/</g, "&lt;");
      inputField += `${key}="${value}" `;
    }
  }

  // Replace all
  inputField = inputField.replace(/ – /g, ":");
  inputField = inputField.replace(/ - /g, ":");

  return inputField + "/>";
}

let table = JSON.parse(fs.readFileSync("FMBFAMILY-FINAL.json", "utf8"));

let html = "<div>\n";
for (let [name, value] of Object.entries(table)) {
  html += `  <div type="maintable" title="${_.escape(name)}">\n`;

  for (let inner_table of value) {
    html += `    <div type="subtable">\n`;
    for (let row of inner_table) {
      // Create input field for each row
      let inputField = createInputField(row);
      html += `      ${inputField}\n`;
    }
    html += `    </div>\n`;
  }

  html += `  </div>\n`;
}
html += "</div>\n";

console.log(html);
