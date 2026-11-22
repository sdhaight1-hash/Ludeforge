// Original dynamic invalidation feasibility model. Run: node research/prototypes/dependency-check.mjs
function experiment(){
 const assert=(ok,msg)=>{if(!ok)throw Error(msg)};
 const clone=x=>JSON.parse(JSON.stringify(x));
 const initial=[{id:"R1",type:"role",f:{max:3}},{id:"R2",type:"role",f:{max:1}},{id:"A",type:"recruit",f:{rank:2,role:"R1"}},{id:"B",type:"recruit",f:{rank:1,role:"R2"}},{id:"D",type:"delegate",f:{influence:3}},{id:"P",type:"proposal",f:{minimum:3}},{id:"L",type:"pledge",f:{delegate:"D",proposal:"P",strength:2}}];
 // Declared predicate AST; dynamic reads tracked during evaluation.
 const constraints=[
  {id:"rank",scope:"recruit",expr:["le",["field","rank"],["ref","role","max"]]},
  {id:"pledge",scope:"pledge",expr:["le",["field","strength"],["ref","delegate","influence"]]},
  {id:"support",scope:"proposal",expr:["le",["field","minimum"],["sum","pledge","proposal","strength"]]}
 ];
 function evaluate(c,o,objs){
  const deps=new Set();
  const read=(id,field)=>{deps.add("f:"+id+":"+field);deps.add("o:"+id);const v=objs.get(id)?.f[field];if(v===undefined)throw Error("missing");return v};
  function go(e){
    if(e[0]==="field")return read(o.id,e[1]);
    if(e[0]==="ref")return read(read(o.id,e[1]),e[2]);
    if(e[0]==="sum"){deps.add("type:"+e[1]);let v=0;for(const x of objs.values())if(x.type===e[1]){if(read(x.id,e[2])===o.id)v+=read(x.id,e[3])}return v}
    if(e[0]==="le"){const a=go(e[1]),b=go(e[2]);if(!Number.isSafeInteger(a)||!Number.isSafeInteger(b))throw Error("kind");return a<=b}
    throw Error("unsupported");
  }
  let result;try{result=go(c.expr)?"ok":"false"}catch{result="error"}return {result,deps:[...deps].sort()};
 }
 // Independent fixture-specific full scan oracle, not same interpreter.
 function oracle(rows){
  const m=new Map(rows.map(o=>[o.id,o]));const out={};
  const compare=(a,b)=>Number.isSafeInteger(a)&&Number.isSafeInteger(b)?(a<=b?"ok":"false"):"error";
  for(const o of rows){
    if(o.type==="recruit")out["rank/"+o.id]=compare(o.f.rank,m.get(o.f.role)?.f.max);
    if(o.type==="pledge")out["pledge/"+o.id]=compare(o.f.strength,m.get(o.f.delegate)?.f.influence);
    if(o.type==="proposal"){let sum=0,bad=false;for(const p of rows.filter(x=>x.type==="pledge")){if(p.f.proposal===undefined)bad=true;if(p.f.proposal===o.id){if(!Number.isSafeInteger(p.f.strength))bad=true;else sum+=p.f.strength}}out["support/"+o.id]=bad?"error":compare(o.f.minimum,sum)}
  }
  return JSON.stringify(Object.entries(out).sort());
 }
 const changes=[
  {op:"set",id:"A",field:"role",value:"R2"},
  {op:"set",id:"R1",field:"max",value:0},
  {op:"set",id:"R2",field:"max",value:3},
  {op:"set",id:"A",field:"role",value:"missing"},
  {op:"add",row:{id:"missing",type:"role",f:{max:5}}},
  {op:"delete",id:"missing"},
  {op:"set",id:"A",field:"role",value:"R1"},
  {op:"add",row:{id:"L2",type:"pledge",f:{delegate:"D",proposal:"P",strength:1}}},
  {op:"set",id:"L2",field:"strength",value:2},
  {op:"add",row:{id:"Q",type:"proposal",f:{minimum:1}}},
  {op:"set",id:"L2",field:"proposal",value:"Q"},
  {op:"delete",id:"L"},
  {op:"set",id:"D",field:"influence",value:0},
  {op:"delete",id:"Q"},
  {op:"delete",id:"L2"},
  {op:"add",row:{id:"C",type:"recruit",f:{rank:1,role:"R1"}}}
 ];
 const objs=new Map(initial.map(o=>[o.id,clone(o)])),cache=new Map(),reverse=new Map();
 function remove(key){const old=cache.get(key);if(old)for(const d of old.deps){const bucket=reverse.get(d);bucket.delete(key);if(!bucket.size)reverse.delete(d)}cache.delete(key)}
 function run(c,o){const key=c.id+"/"+o.id;remove(key);const v=evaluate(c,o,objs);cache.set(key,v);for(const d of v.deps){if(!reverse.has(d))reverse.set(d,new Set());reverse.get(d).add(key)}}
 for(const o of objs.values())for(const c of constraints)if(c.scope===o.type)run(c,o);
 const snapshot=()=>JSON.stringify([...cache].map(([k,v])=>[k,v.result]).sort());
 assert(snapshot()===oracle([...objs.values()]),"initial");
 const results=[];
 for(const change of changes){
  const dirty=new Set();
  const invalidate=k=>{for(const id of reverse.get(k)||[])dirty.add(id)};
  if(change.op==="set"){invalidate("f:"+change.id+":"+change.field);objs.get(change.id).f[change.field]=change.value}
  if(change.op==="add"){const o=clone(change.row);invalidate("o:"+o.id);invalidate("type:"+o.type);objs.set(o.id,o);for(const c of constraints)if(c.scope===o.type)dirty.add(c.id+"/"+o.id)}
  if(change.op==="delete"){const o=objs.get(change.id);invalidate("o:"+o.id);invalidate("type:"+o.type);for(const c of constraints)if(c.scope===o.type)dirty.add(c.id+"/"+o.id);objs.delete(o.id)}
  for(const key of dirty){const [id,obj]=key.split("/");const c=constraints.find(x=>x.id===id),o=objs.get(obj);if(o)run(c,o);else remove(key)}
  assert(snapshot()===oracle([...objs.values()]),"parity "+JSON.stringify(change));
  results.push({change,reevaluated:dirty.size,activeConstraints:cache.size,parity:true});
 }
 return {initialParity:true,editParity:results.length,results,limitations:"Original tiny finite synthetic model. Dynamic dependencies and membership keys; independent fixture-specific full scan oracle. Constraints use trial AST with no type compiler, no arbitrary queries, no compound uniqueness, no concurrent writes, no persistence or measured performance. Intermediate candidate states intentionally include dangling refs to test error propagation; a production commit must reject invalid candidate states."};
}
console.log(JSON.stringify(experiment(),null,2));
