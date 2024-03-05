import wtf from "wtf_wikipedia";
import fs from "fs";

await fetchTemplates("FMP100", "{{Template:FMP100 Parameter list}}");
await fetchTemplates("FMBFAMILY", "{{Template:FMB Device Family Parameter list}}");

async function fetchTemplates(name, template) {
  let parameter_list = await fetch(`https://wiki.teltonika-gps.com/api.php?action=expandtemplates&text=${template}&format=json&prop=wikitext`);
  let parameter_list_json = await parameter_list.json();
  let wikitext = parameter_list_json.expandtemplates.wikitext;

  // Parse
  let doc = wtf(wikitext);
  let json = doc.json();

  // Write to file
  fs.writeFileSync(`${name}.json`, JSON.stringify(json, null, 2));
}
