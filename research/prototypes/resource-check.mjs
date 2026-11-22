// Original bounded resource model. Run: node research/prototypes/resource-check.mjs
function study(){
 const initial={stock:6,work:[0,0],product:0,waste:0,clock:0};
 const ops=["start0","start1","tick","finish0","finish1","cancel0","cancel1"];
 function step(s,op,mutant="none"){
  const n={...s,work:[...s.work]};const i=Number(op.at(-1));
  if(op.startsWith("start")&&s.work[i]===0&&s.stock>=4){n.stock-=4;n.work[i]=2;return n}
  if(op==="tick"&&s.clock<4){n.clock++;n.work=n.work.map(x=>x>1?x-1:x);return n}
  if(op.startsWith("finish")&&s.work[i]===1){n.work[i]=0;n.product+=3;n.waste+=mutant==="duplicate"?2:1;return n}
  if(op.startsWith("cancel")&&s.work[i]>0){n.stock+=mutant==="refund"?5:4;n.work[i]=0;return n}
  return s
 }
 const invariant=s=>s.stock>=0&&s.product>=0&&s.waste>=0&&s.stock+4*s.work.filter(x=>x>0).length+s.product+s.waste===6;
 function explore(mutant){let seen=new Set([JSON.stringify(initial)]),q=[{s:initial,path:[]}],transitions=0;
  for(let i=0;i<q.length;i++){const {s,path}=q[i];for(const op of ops){const n=step(s,op,mutant),p=[...path,op];transitions++;if(!invariant(n))return {states:seen.size,transitions,counterexample:p,state:n};const k=JSON.stringify(n);if(!seen.has(k)){seen.add(k);q.push({s:n,path:p})}}}
  return {states:seen.size,transitions,counterexample:null};
 }
 const baseline=explore("none"),mutants={duplicate:explore("duplicate"),refund:explore("refund")};
 if(baseline.counterexample||Object.values(mutants).some(x=>!x.counterexample))throw Error("conservation");
 // Same snapshot requests, explicit stable arbitration. A rejected competitor gets no resource.
 function allocate(stock,requests){let remaining=stock;const accepted=[],rejected=[];for(const r of [...requests].sort((a,b)=>b.priority-a.priority||(a.id<b.id?-1:a.id>b.id?1:0))){if(r.cost<=remaining){remaining-=r.cost;accepted.push(r.id)}else rejected.push(r.id)}return {remaining,accepted,rejected}}
 const requests=[{id:"A",priority:1,cost:4},{id:"B",priority:1,cost:4},{id:"C",priority:0,cost:2}];
 const batch=allocate(6,requests);let permutations=0;
 for(const a of requests)for(const b of requests.filter(r=>r!==a)){const c=requests.find(r=>r!==a&&r!==b);if(JSON.stringify(allocate(6,[a,b,c]))!==JSON.stringify(batch))throw Error("order");permutations++}
 // Bounded reachability exploit search: A->B->A round trip is exploit only if value increases AND repeatable.
 function search(gain,maxDepth){const initial=[1,0],q=[{v:initial,path:[]}],seen=new Set([initial.join(",")]);for(let i=0;i<q.length;i++){const {v,path}=q[i];if(path.length&&v[0]>initial[0]&&v[1]===0)return {witness:path,state:v};if(path.length===maxDepth)continue;for(const [name,input,delta] of [["buy",0,[-1,1]],["sell",1,[gain,-1]]]){if(v[input]===0)continue;const n=v.map((x,j)=>x+delta[j]),k=n.join(",");if(!seen.has(k)){seen.add(k);q.push({v:n,path:[...path,name]})}}}return {witness:null,states:seen.size}}
 const balanced=search(1,8),exploit=search(2,8);if(balanced.witness||!exploit.witness)throw Error("exploit search");
 return {baseline,mutants,batch,permutations,balanced,exploit,limitations:"Finite abstract states, two workpieces, bounded time, explicit fixed integer material accounting; no real scheduler, timestamps, arbitrary agents, starvation/liveness proof, persistence, general exploit detection, performance or Rust validation."}
}
console.log(JSON.stringify(study(),null,2));
