// Original bounded research; not a production implementation.
// node research/prototypes/ontology-check.mjs
function experiment(){
 const assert=(ok,msg)=>{if(!ok)throw Error(msg)};
 const clone=x=>JSON.parse(JSON.stringify(x));
 const outcomes=[];
 const check=(name,test)=>{assert(test(),name);outcomes.push(name)};
 // Constraints as intersected finite domains, explicit identity independent of names.
 const resolve=(components)=>{
   const m=new Map();
   for(const c of components){for(const f of c.fields){
     const a=m.get(f.id);
     if(!a)m.set(f.id,{...f});
     else{
       if(a.kind!==f.kind)throw Error("kind-conflict");
       if(f.kind==="int"){a.min=Math.max(a.min,f.min);a.max=Math.min(a.max,f.max);if(a.min>a.max)throw Error("empty-domain")}
       else if(f.kind==="ref"&&a.target!==f.target)throw Error("target-conflict");
       a.required=a.required||f.required;
     }
   }}
   return [...m.values()].sort((a,b)=>a.id<b.id?-1:a.id>b.id?1:0);
 };
 const f=(id,min,max,required=true)=>({id,kind:"int",min,max,required});
 const common=[{id:"base",fields:[f("rank",1,5,false),f("focus",0,10)]},{id:"advanced",fields:[f("rank",2,4),f("focus",2,8)]},{id:"cap",fields:[f("rank",3,5)]}];
 const canonical=x=>JSON.stringify(x);
 const permutations=a=>a.length===0?[[]]:a.flatMap((x,i)=>permutations(a.filter((_,j)=>i!==j)).map(y=>[x,...y]));
 const result=resolve(common);
 for(const [i,p] of permutations(common).entries())check("composition permutation "+i,()=>canonical(resolve(p))===canonical(result));
 check("composition idempotent",()=>canonical(resolve([...common,common[0]]))===canonical(result));
 check("intersection expected",()=>result.find(x=>x.id==="rank").min===3&&result.find(x=>x.id==="rank").max===4&&result.find(x=>x.id==="focus").min===2);
 const throws=(fn,msg)=>{try{fn();return false}catch(e){return e.message===msg}};
 check("reject empty intersection",()=>throws(()=>resolve([...common,{fields:[f("rank",5,7)]}]),"empty-domain"));
 check("reject mismatched field kinds",()=>throws(()=>resolve([...common,{fields:[{id:"rank",kind:"text",required:true}]}]),"kind-conflict"));
 check("reject incompatible reference target",()=>throws(()=>resolve([{fields:[{id:"ref",kind:"ref",target:"A"}]},{fields:[{id:"ref",kind:"ref",target:"B"}]}]),"target-conflict"));
 // Generic predicate interpreter in a restricted toy data domain.
 const read=(field)=>({op:"read",field});const ref=(field,property)=>({op:"refRead",field,property});const num=value=>({op:"int",value});const le=(a,b)=>({op:"le",a,b});
 const constraints=[{id:"rank_limit",type:"recruit",expr:le(read("rank"),ref("role","max"))},{id:"pledge_limit",type:"pledge",expr:le(read("strength"),ref("delegate","influence"))}];
 const unique=[{id:"one_pledge",type:"pledge",fields:["delegate","proposal"]}];
 function evalExpr(e,o,index){
   if(e.op==="int")return e.value;
   if(e.op==="read"||e.op==="refRead"){
     const v=e.op==="read"?o.fields[e.field]:index.get(o.fields[e.field])?.fields[e.property];
     if(!Number.isSafeInteger(v))throw Error("missing-or-invalid-read");return v;
   }
   if(e.op==="le")return evalExpr(e.a,o,index)<=evalExpr(e.b,o,index);
   throw Error("unknown-op");
 }
 function validate(objects){
   const index=new Map(objects.map(o=>[o.id,o]));let errors=[];
   for(const c of constraints)for(const o of objects.filter(x=>x.type===c.type)){
     try{if(evalExpr(c.expr,o,index)!==true)errors.push(c.id+":"+o.id)}catch{errors.push("evaluation:"+c.id+":"+o.id)}
   }
   for(const u of unique){const groups=new Map();for(const o of objects.filter(x=>x.type===u.type)){const k=canonical(u.fields.map(f=>o.fields[f]));if(!groups.has(k))groups.set(k,[]);groups.get(k).push(o.id)}
     for(const ids of groups.values())if(ids.length>1)errors.push(u.id+":"+ids.sort().join(","));
   }
   return errors.sort();
 }
 const data=[{id:"R",type:"role",fields:{max:3}},{id:"N",type:"recruit",fields:{rank:2,role:"R"}},{id:"D",type:"delegate",fields:{influence:2}},{id:"P",type:"proposal",fields:{}},{id:"L",type:"pledge",fields:{strength:2,delegate:"D",proposal:"P"}}];
 check("valid predicates",()=>validate(data).length===0);
 check("cross-object rank constraint",()=>{const p=clone(data);p[0].fields.max=1;return validate(p).includes("rank_limit:N")});
 check("cross-object pledge constraint",()=>{const p=clone(data);p[2].fields.influence=1;return validate(p).includes("pledge_limit:L")});
 check("missing referenced value fails closed",()=>{const p=clone(data);delete p[0].fields.max;return validate(p).includes("evaluation:rank_limit:N")});
 check("composite unique key",()=>{const p=clone(data);p.push({...p[4],id:"L2"});return validate(p).includes("one_pledge:L,L2")});
 check("stable reordered diagnostics",()=>{const p=clone(data);p.push({...p[4],id:"L2"});p[2].fields.influence=1;return canonical(validate(p))===canonical(validate(p.reverse()))});
 // Compare reverse dependency invalidation with full scan across bounded edits.
 const deps=new Map([["R/max",["N"]],["N/rank",["N"]],["D/influence",["L"]],["L/strength",["L"]]]);
 const edits=[["R","max"],["N","rank"],["D","influence"],["L","strength"]];
 let parity=0;
 for(const [id,key] of edits)for(let value=0;value<=5;value++){
   const p=clone(data);p.find(x=>x.id===id).fields[key]=value;
   const affected=new Set(deps.get(id+"/"+key));
   const before=validate(data),after=validate(p);
   const incremental=[...before.filter(e=>!affected.has(e.split(":").at(-1))),...after.filter(e=>affected.has(e.split(":").at(-1)))].sort();
   assert(canonical(incremental)===canonical(after),"invalidation mismatch");parity++;
 }
 // Typed relation topology with independent edge semantics.
 function relValidate(edges,{symmetric=false,uniquePair=true,acyclic=false,maxOut=null,self=false}={}){
   const seen=new Set(),out=new Map(),adj=new Map(),nodes=new Set();
   for(const [x,y] of edges){
     if(!self&&x===y)return "self";
     const p=symmetric?[x,y].sort():[x,y];const k=canonical(p);
     if(uniquePair&&seen.has(k))return "duplicate";
     seen.add(k);nodes.add(x);nodes.add(y);
     for(const [a,b] of symmetric?[[x,y],[y,x]]:[[x,y]]){
       out.set(a,(out.get(a)||0)+1);if(maxOut!==null&&out.get(a)>maxOut)return "cardinality";
       if(!adj.has(a))adj.set(a,[]);adj.get(a).push(b);
     }
   }
   if(acyclic){const visited=new Set(),active=new Set();function visit(v){if(active.has(v))return false;if(visited.has(v))return true;active.add(v);for(const w of adj.get(v)||[])if(!visit(w))return false;active.delete(v);visited.add(v);return true}for(const n of nodes)if(!visit(n))return "cycle"}
   return "ok";
 }
 check("symmetric duplicate rejected",()=>relValidate([["A","B"],["B","A"]],{symmetric:true})==="duplicate");
 check("directed reciprocal valid",()=>relValidate([["A","B"],["B","A"]])==="ok");
 check("ancestry cycle rejected",()=>relValidate([["A","B"],["B","C"],["C","A"]],{acyclic:true})==="cycle");
 check("intentional semantic cycle accepted",()=>relValidate([["A","B"],["B","C"],["C","A"]])==="ok");
 check("exclusive cardinality rejects not replaces",()=>relValidate([["A","B"],["A","C"]],{maxOut:1})==="cardinality");
 check("self link rejected",()=>relValidate([["A","A"]])==="self");
 // Module lock: exact identity + revision, explicit collision policy, no implicit replacement.
 function flatten(modules,locks){
   const byKey=new Map(modules.map(m=>[m.id+"@"+m.revision,m]));
   const fields=new Map(),used=new Set();
   for(const lock of locks){if(used.has(lock.id))throw Error("multiple-revisions");used.add(lock.id);const m=byKey.get(lock.id+"@"+lock.revision);if(!m)throw Error("missing-revision");for(const field of m.fields){if(fields.has(field.id))throw Error("id-collision");fields.set(field.id,field)}}
   return [...fields.keys()].sort();
 }
 const modules=[{id:"a",revision:"r1",fields:[{id:"a_rank"}]},{id:"a",revision:"r2",fields:[{id:"a_rank"},{id:"a_focus"}]},{id:"b",revision:"r1",fields:[{id:"b_repute"}]},{id:"c",revision:"r1",fields:[{id:"a_rank"}]}];
 const lock=(id,revision)=>({id,revision});
 check("pinned module excludes future field",()=>canonical(flatten(modules,[lock("a","r1"),lock("b","r1")]))===canonical(["a_rank","b_repute"]));
 check("module load order irrelevant",()=>canonical(flatten(modules,[lock("a","r1"),lock("b","r1")]))===canonical(flatten(modules,[lock("b","r1"),lock("a","r1")])));
 check("missing pinned revision rejects",()=>throws(()=>flatten(modules,[lock("a","r0")]),"missing-revision"));
 check("module collision rejects",()=>throws(()=>flatten(modules,[lock("a","r1"),lock("c","r1")]),"id-collision"));
 check("multiple module revisions reject",()=>throws(()=>flatten(modules,[lock("a","r1"),lock("a","r2")]),"multiple-revisions"));
 // Simulate copy-on-write migration with identity reservation.
 const old={revision:1,fields:[{id:"power",label:"Power",min:0,max:5}],records:[{id:"x",values:{power:4}}],retired:[]};
 function migrate(p,ops){
   const n=clone(p);
   for(const op of ops){
     if(op.kind==="rename"){const field=n.fields.find(f=>f.id===op.id);if(!field)throw Error("missing-field");field.label=op.label}
     else if(op.kind==="narrow"){const field=n.fields.find(f=>f.id===op.id);if(!field)throw Error("missing-field");field.max=op.max;if(n.records.some(r=>r.values[op.id]>op.max))throw Error("out-of-domain")}
     else if(op.kind==="delete"){if(n.records.some(r=>Object.hasOwn(r.values,op.id)))throw Error("live-data");n.fields=n.fields.filter(f=>f.id!==op.id);n.retired.push(op.id)}
     else if(op.kind==="add"){if(n.retired.includes(op.id)||n.fields.some(f=>f.id===op.id))throw Error("reserved-id");n.fields.push({id:op.id,label:op.id,min:0,max:5})}
     else throw Error("unknown-migration");
   }
   n.revision++;return n;
 }
 const untouched=canonical(old);
 check("rename preserves values",()=>migrate(old,[{kind:"rename",id:"power",label:"Might"}]).records[0].values.power===4&&canonical(old)===untouched);
 check("incompatible narrowing atomic rollback",()=>throws(()=>migrate(old,[{kind:"rename",id:"power",label:"Might"},{kind:"narrow",id:"power",max:3}]),"out-of-domain")&&canonical(old)===untouched);
 check("live field deletion rejects",()=>throws(()=>migrate(old,[{kind:"delete",id:"power"}]),"live-data"));
 const empty=clone(old);empty.records=[];
 check("retired field cannot be repurposed",()=>throws(()=>migrate(empty,[{kind:"delete",id:"power"},{kind:"add",id:"power"}]),"reserved-id"));
 return {assertions:outcomes.length,checks:outcomes,incrementalParityCases:parity,compositionResult:result,limitations:"Synthetic JS research only. Hand-authored reverse index, finite integer intersection, limited predicate ops, tiny relation graphs and flat module locks; no parser, compiler, transitive imports, durable storage, security proof or production performance."};
}
console.log(JSON.stringify(experiment(),null,2));
