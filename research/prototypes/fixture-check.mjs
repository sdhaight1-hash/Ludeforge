// Run from anywhere: node research/prototypes/fixture-check.mjs
import {readFileSync} from "node:fs";
function fixtureExperiment(fixtures){
  const clone=x=>JSON.parse(JSON.stringify(x));const assert=(b,m)=>{if(!b)throw Error(m)};
  function validate(p){
    const errors=[], ids=new Set(), types=new Map(), objs=new Map(), rels=new Map(), pairs=new Set();
    const err=(code)=>errors.push(code);
    const id=x=>{if(typeof x!=="string"||!/^[A-Za-z][A-Za-z0-9_-]{0,63}$/.test(x))err("ID");else if(ids.has(x))err("DUPLICATE");else ids.add(x)};
    if(p.format_version!==1)err("VERSION");
    for(const t of p.types){id(t.id);types.set(t.id,t);for(const f of t.fields){id(f.id);if(f.kind==="enum")for(const o of f.options)id(o.id)}}
    for(const o of p.objects){id(o.id);objs.set(o.id,o)}
    for(const t of p.types)for(const f of t.fields)if(f.kind==="ref"&&!types.has(f.target))err("TARGET_TYPE");
    for(const o of p.objects){
      const t=types.get(o.type);if(!t){err("TYPE");continue}
      for(const k of Object.keys(o.values))if(!t.fields.some(f=>f.id===k))err("UNKNOWN_FIELD");
      for(const f of t.fields){
        const v=o.values[f.id];if(v===undefined){if(f.required)err("REQUIRED");continue}
        if(!v||v.kind!==f.kind){err("KIND");continue}
        if(v.kind==="int"&&(!Number.isSafeInteger(v.value)||v.value<f.min||v.value>f.max))err("RANGE");
        if(v.kind==="enum"&&!f.options.some(x=>x.id===v.value))err("ENUM");
        if(v.kind==="ref"&&objs.get(v.value)?.type!==f.target)err("REFERENCE");
      }
    }
    for(const r of p.relation_types){id(r.id);rels.set(r.id,r);if(!types.has(r.source)||!types.has(r.target))err("TARGET_TYPE")}
    for(const e of p.relations){id(e.id);const t=rels.get(e.type);
      if(!t||objs.get(e.source)?.type!==t.source||objs.get(e.target)?.type!==t.target){err("ENDPOINT");continue}
      const pair=JSON.stringify([e.type,e.source,e.target]);if(t.unique_pair&&pairs.has(pair))err("PAIR");pairs.add(pair);
    }
    if(p.rules.length)err("UNSUPPORTED_RULES");
    return errors.sort();
  }
  function normalize(x){if(Array.isArray(x))return x.map(normalize);if(x!==null&&typeof x==="object")return Object.fromEntries(Object.keys(x).sort().map(k=>[k,normalize(x[k])]));return x}
  function bytes(p){const q=clone(p);for(const k of ["types","objects","relation_types","relations"])q[k].sort((a,b)=>a.id<b.id?-1:a.id>b.id?1:0);for(const t of q.types){t.fields.sort((a,b)=>a.id<b.id?-1:a.id>b.id?1:0)}return JSON.stringify(normalize(q))}
  const outcomes=[];
  for(const [name,p] of Object.entries(fixtures)){
    assert(validate(p).length===0,name+" valid");
    const q=clone(p);q.types.reverse();q.objects.reverse();q.relations.reverse();assert(bytes(q)===bytes(p),name+" unordered normalization");assert(bytes(JSON.parse(bytes(p)))===bytes(p),name+" round trip");
    const rename=clone(p);rename.objects[0].label="Renamed";assert(validate(rename).length===0,name+" rename");
    outcomes.push({name,types:p.types.length,objects:p.objects.length,relations:p.relations.length,valid:true,roundTrip:true,rename:true,normalizedReorder:true});
  }
  const mutations=[
    ["version","lantern",p=>p.format_version=2,"VERSION"],
    ["duplicate","lantern",p=>p.objects.push(clone(p.objects[0])),"DUPLICATE"],
    ["wrong type","lantern",p=>p.objects[1].type="missing","TYPE"],
    ["required","lantern",p=>delete p.objects[1].values.focus,"REQUIRED"],
    ["unknown field","lantern",p=>p.objects[1].values.extra={kind:"int",value:0},"UNKNOWN_FIELD"],
    ["kind","lantern",p=>p.objects[1].values.focus={kind:"text",value:"3"},"KIND"],
    ["range","lantern",p=>p.objects[1].values.rank.value=4,"RANGE"],
    ["dangling","lantern",p=>p.objects[1].values.role_ref.value="missing","REFERENCE"],
    ["wrong reference kind","lantern",p=>p.objects[1].values.role_ref.value="first_trial","REFERENCE"],
    ["enum","council",p=>p.objects[2].values.status.value="missing","ENUM"],
    ["endpoint","council",p=>p.relations[0].source="bridge","ENDPOINT"],
    ["duplicate pair","council",p=>p.relations.push({...p.relations[0],id:"new_edge"}),"PAIR"],
    ["unsupported rule","council",p=>p.rules.push({id:"rule"}),"UNSUPPORTED_RULES"]
  ].map(([name,fixture,mutate,expected])=>{const p=clone(fixtures[fixture]);mutate(p);const actual=validate(p);assert(actual.includes(expected),name);return {name,expected,actual}});
  return {outcomes,mutations,limitations:"Narrow prototype, structured input only. No duplicate JSON key detection, general schema validation, arbitrary constraints, runtime rules, atomic writes, Unicode canonicalization or production benchmark. Predicate constraints in fixture prose remain future executable cases."};
}
const read=p=>JSON.parse(readFileSync(new URL(p,import.meta.url),"utf8"));
console.log(JSON.stringify(fixtureExperiment({lantern:read("../../examples/lantern-academy/project.json"),council:read("../../examples/council-of-reeds/project.json")}),null,2));
