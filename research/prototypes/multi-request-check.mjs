// Original bounded Phase 0 study. Run: node research/prototypes/multi-request-check.mjs
function protocolStudy(){
  const initial={stock:2,rev:0,requests:[null,null],events:[]};
  const ops=["request0","request1","alter0","alter1","approve0","approve1","deny0","deny1","cancel0","cancel1","untrusted0","untrusted1","edit"];
  const clone=s=>JSON.parse(JSON.stringify(s));
  const key=s=>JSON.stringify(s);
  function step(s,op,mutant){
    const next=clone(s);
    if(op==="edit"){
      if(s.events.some(e=>e.id==="edit"))return s;
      next.events.push({id:"edit",before:s.rev,delta:0,kind:"edit"});next.rev++;return next;
    }
    const i=Number(op.at(-1)),r=s.requests[i],action=op.slice(0,-1);
    if(action==="request"&&r===null){next.requests[i]={base:s.rev,status:"pending",payload:"one"};return next}
    if(action==="alter"&&r&&mutant==="payload"){next.requests[i].payload="two";return next}
    if(action==="untrusted"&&r?.status==="pending"&&mutant==="authority")next.requests[i].status="cancelled";
    else if(action==="cancel"&&r?.status==="pending"){next.requests[i].status="cancelled";return next}
    else if((action==="approve"||action==="deny")&&r?.status==="pending"){
      if(r.base!==s.rev&&mutant!=="stale"){next.requests[i].status="stale";return next}
      if(action==="approve"&&s.stock===0)return s;
      const delta=action==="approve"?-1:0;next.stock+=delta;next.rev++;next.requests[i].status=action==="approve"?"approved":"denied";next.events.push({id:String(i),before:s.rev,delta,kind:action});return next;
    }else if(action==="approve"&&r?.status==="approved"&&mutant==="duplicate"){
      next.stock--;next.rev++;next.events.push({id:String(i),before:s.rev,delta:-1,kind:action});return next;
    }else return s;
    return next;
  }
  function invariant(s,n,op){
    let stock=2,rev=0;const eventIds=new Set();
    for(const e of n.events){
      if(e.before!==rev||eventIds.has(e.id))return "journal order or duplicate";
      rev++;stock+=e.delta;eventIds.add(e.id);
    }
    if(stock!==n.stock||rev!==n.rev||stock<0||stock>2)return "journal/state mismatch";
    for(let i=0;i<2;i++){
      const r=n.requests[i],e=n.events.find(e=>e.id===String(i));
      if((r?.status==="approved"||r?.status==="denied")!==Boolean(e))return "terminal receipt mismatch";
      if(r?.payload!=="one"&&r!==null)return "payload mutated";
      if(s.requests[i]&&r?.base!==s.requests[i].base)return "checkpoint overwritten";
      if(r&&(r.status==="approved"||r.status==="denied")&&e?.before!==r.base)return "stale resolution committed";
    }
    if(op.startsWith("untrusted")&&key(s)!==key(n))return "unauthorized mutation";
    if(op.startsWith("alter")&&key(s)!==key(n))return "conflicting payload accepted";
    if(op.startsWith("request")||op.startsWith("cancel")){
      if(s.stock!==n.stock||s.rev!==n.rev||key(s.events)!==key(n.events))return "administrative effect changed mechanics";
    }
    return null;
  }
  function explore(mutant){
    const seen=new Set([key(initial)]),q=[{s:initial,trace:[]}];let transitions=0;
    for(let i=0;i<q.length;i++){
      const {s,trace}=q[i];for(const op of ops){const next=step(s,op,mutant),path=[...trace,op];transitions++;const violation=invariant(s,next,op);if(violation)return {states:seen.size,transitions,witness:path,violation};const k=key(next);if(!seen.has(k)){seen.add(k);q.push({s:next,trace:path})}}
    }
    return {states:seen.size,transitions,witness:null};
  }
  const baseline=explore("none"),mutants=Object.fromEntries(["stale","duplicate","authority","payload"].map(m=>[m,explore(m)]));
  if(baseline.witness||Object.values(mutants).some(x=>!x.witness))throw Error("unexpected model result");
  return {baseline,mutants,limitations:"Bounded in-memory toy model, two unique fixed payload request IDs, one optional external edit, serial transitions, no process crash, rule evaluation, cryptographic authentication, concurrent IO or real persistence. Journal application tests mechanical state, not full decision re-execution."};
}
console.log(JSON.stringify(protocolStudy(),null,2));
