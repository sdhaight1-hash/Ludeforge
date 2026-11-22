// Original bounded research model, not production engine. Run: node research/prototypes/kernel-check.mjs
function experiment(){
  const assert=(ok,msg)=>{if(!ok)throw Error(msg)};
  const key=x=>JSON.stringify(x);
  const initial={rev:0,stock:2,pending:null,done:false,events:0};
  const ops=["request","edit","approve","deny","unauthorized","retry","invalid"];
  function step(s,op,mutant){
    const n={...s};
    if(op==="request"&&!s.pending&&!s.done&&s.stock>0){n.pending=s.rev;return n}
    if(op==="edit"&&s.rev<3){n.rev++;return n}
    if(op==="invalid"&&mutant==="partial"){n.stock--;return n}
    if(op==="retry"&&s.done&&mutant==="duplicate"){n.stock--;n.events++;return n}
    if(op==="approve"&&s.pending!==null&&!s.done&&s.stock>0&&(s.pending===s.rev||mutant==="stale")){n.pending=null;n.done=true;n.stock--;n.events++;n.rev++;return n}
    if(op==="deny"&&s.pending!==null&&s.pending===s.rev){n.pending=null;n.done=true;n.events++;n.rev++;return n}
    if(op==="unauthorized"&&mutant==="authority"&&s.pending!==null){n.pending=null;n.done=true;n.events++;n.rev++;return n}
    return s;
  }
  function violation(s,n,op){
    if(n.stock<0||n.stock>2||n.events>1)return "bounds/exactly-once";
    if(["invalid","unauthorized","retry"].includes(op)&&key(s)!==key(n))return "rejected action mutated state";
    if(op==="approve"&&s.pending!==s.rev&&key(s)!==key(n))return "stale approval committed";
    if(op==="request"&&(s.stock!==n.stock||s.events!==n.events||s.rev!==n.rev))return "checkpoint committed effects";
    if(op==="approve"&&key(s)!==key(n)&&(s.stock-n.stock!==1||n.events-s.events!==1||!n.done))return "non-atomic commit";
    return null;
  }
  function explore(mutant){
    const q=[{s:initial,path:[]}], seen=new Set([key(initial)]);let transitions=0;
    for(let i=0;i<q.length;i++){const {s,path}=q[i];for(const op of ops){const n=step(s,op,mutant);transitions++;const why=violation(s,n,op);if(why)return {states:seen.size,transitions,counterexample:[...path,op],why};const k=key(n);if(!seen.has(k)){seen.add(k);q.push({s:n,path:[...path,op]})}}}
    return {states:seen.size,transitions,counterexample:null};
  }
  const protocol=Object.fromEntries(["baseline","partial","duplicate","stale","authority"].map(m=>[m,explore(m)]));
  assert(protocol.baseline.counterexample===null,"baseline unsafe");
  for(const m of ["partial","duplicate","stale","authority"])assert(protocol[m].counterexample!==null,`mutant survived ${m}`);
  function dp(faces){let d=new Map([[0,1]]);for(const f of faces){const e=new Map();for(const [sum,c] of d)for(let j=1;j<=f;j++)e.set(sum+j,(e.get(sum+j)||0)+c);d=e}return d}
  function brute(faces){const d=new Map();function visit(i,s){if(i===faces.length){d.set(s,(d.get(s)||0)+1);return}for(let j=1;j<=faces[i];j++)visit(i+1,s+j)}visit(0,0);return d}
  let bins=0,thresholds=0; const probability=[];
  for(const faces of [[20],[6,6],[6,6,6],[10,10],[4,6,8],[100]]){
    const a=dp(faces),b=brute(faces);let tot=faces.reduce((x,y)=>x*y,1);
    assert(a.size===b.size,"support");for(const [k,v] of a){assert(v===b.get(k),"bin");bins++}
    assert([...a.values()].reduce((x,y)=>x+y,0)===tot,"mass");
    for(let t=0;t<=faces.reduce((x,y)=>x+y,0)+1;t++){assert([...a].filter(([k])=>k>=t).reduce((x,[,v])=>x+v,0)===[...b].filter(([k])=>k>=t).reduce((x,[,v])=>x+v,0),"tail");thresholds++}
    probability.push({faces,outcomes:tot,bins:a.size});
  }
  assert([...dp([20])].filter(([k])=>k+2>=12).reduce((x,[,v])=>x+v,0)===11,"Lantern expected");
  assert(10+2+3+Math.max(1,4)===19,"Glass Market expected");
  assert(2+3+1===6,"Tidemill expected");
  return {protocol,probability,bins,thresholds,fixtureChecks:3,limitations:"Finite abstract state machine, one request, coarse revisions, no persistence/concurrency; dice sums only, independent uniform outcomes and small safe integer counts. Not production tests."};
}
console.log(JSON.stringify(experiment(),null,2));
