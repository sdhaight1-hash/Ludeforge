// Original mathematical feasibility study. Run: node research/prototypes/math-check.mjs
function study(){
 const checks=[];function test(n,b){if(!b)throw Error(n);checks.push(n)}
 const gcd=(a,b)=>{a=a<0n?-a:a;while(b){[a,b]=[b,a%b]}return a<0n?-a:a};
 function rat(a,b=1){a=BigInt(a);b=BigInt(b);if(!b)throw Error("zero");if(b<0n){a=-a;b=-b}const g=gcd(a,b);return [a/g,b/g]}
 const add=(a,b)=>rat(a[0]*b[1]+b[0]*a[1],a[1]*b[1]),mul=(a,b)=>rat(a[0]*b[0],a[1]*b[1]),div=(a,b)=>rat(a[0]*b[1],a[1]*b[0]),fmt=a=>a.join("/");
 function round(a,mode){const [n,d]=a,q=n/d,r=n%d;if(!r)return q;const sign=n<0n?-1n:1n,abs=r<0n?-r:r;if(mode==="zero")return q;if(mode==="floor")return n<0n?q-1n:q;if(mode==="ceil")return n>0n?q+1n:q;if(mode==="even")return abs*2n>d||(abs*2n===d&&q%2n!==0n)?q+sign:q;if(mode==="away")return abs*2n>=d?q+sign:q;throw Error("mode")}
 const rejects=(f,msg)=>{try{f();return false}catch(e){return e.message===msg}};
 test("normalized negative denominator",fmt(rat(6,-8))==="-3/4");test("exact decimal addition",fmt(add(rat(1,10),rat(2,10)))==="3/10");test("exact thirds",fmt(mul(rat(1,3),rat(3)))==="1/1");test("zero divisor rejected",rejects(()=>div(rat(1),rat(0)),"zero"));
 for(const [n,d,mode,e] of [[-3,2,"floor",-2],[-3,2,"ceil",-1],[-3,2,"zero",-1],[-5,2,"even",-2],[-3,2,"even",-2],[5,2,"even",2],[3,2,"even",2],[-5,2,"away",-3],[5,2,"away",3]])test(`round ${n}/${d} ${mode}`,round(rat(n,d),mode)===BigInt(e));
 const units={g:["mass",1],kg:["mass",1000],s:["time",1],min:["time",60],silver:["silver",1],mana:["mana",1],m:["length",1]};
 const q=(n,u)=>({v:mul(rat(n),rat(units[u][1])),d:{[units[u][0]]:1}});
 const key=d=>Object.entries(d).filter(x=>x[1]).sort(([a],[b])=>a<b?-1:a>b?1:0).map(x=>x.join(":")).join(",");
 function qa(a,b){if(key(a.d)!==key(b.d))throw Error("dimension");return {v:add(a.v,b.v),d:a.d}}
 function qd(a,b){const d={...a.d};for(const [k,v] of Object.entries(b.d))d[k]=(d[k]||0)-v;return {v:div(a.v,b.v),d}}
 test("mixed mass units",fmt(qa(q(2,"kg"),q(350,"g")).v)==="2350/1");test("mass/time rejected",rejects(()=>qa(q(1,"kg"),q(1,"min")),"dimension"));test("currency/mana rejected",rejects(()=>qa(q(1,"silver"),q(1,"mana")),"dimension"));const speed=qd(q(120,"m"),q(2,"min"));test("rate",key(speed.d)==="length:1,time:-1"&&fmt(speed.v)==="1/1");
 const entries=[{id:"a",stage:"add",n:2},{id:"b",stage:"add",n:3},{id:"c",stage:"best",n:4},{id:"d",stage:"best",n:4},{id:"e",stage:"mul",n:3,d:2},{id:"f",stage:"cap",n:20}];
 function mods(es,stages){let v=rat(10),receipt=[];for(const stage of stages){const e=es.filter(x=>x.stage===stage).sort((a,b)=>a.id<b.id?-1:a.id>b.id?1:0);if(stage==="add")for(const x of e){v=add(v,rat(x.n));receipt.push(x.id)}if(stage==="best"&&e.length){const x=e.reduce((a,b)=>b.n>a.n?b:a);v=add(v,rat(x.n));receipt.push(x.id)}if(stage==="mul")for(const x of e){v=mul(v,rat(x.n,x.d));receipt.push(x.id)}if(stage==="cap"&&e.length){const x=e[0];if(v[0]>BigInt(x.n)*v[1])v=rat(x.n);receipt.push(x.id)}}return {v:fmt(v),receipt}}
 const stages=["add","best","mul","cap"],m=mods(entries,stages);
 test("explicit modifier phases and tie",m.v==="20/1"&&m.receipt.includes("c")&&!m.receipt.includes("d"));test("storage order irrelevant",JSON.stringify(m)===JSON.stringify(mods([...entries].reverse(),stages)));test("semantic stage order changes value",mods(entries,["add","best","cap","mul"]).v==="57/2");
 const acc=(m,k,w)=>m.set(k,(m.get(k)||0n)+w),pow=(a,b)=>BigInt(a)**BigInt(b);
 function C(n,k){if(k<0||k>n)return 0n;let r=1n;for(let i=1;i<=Math.min(k,n-k);i++)r=r*BigInt(n-i+1)/BigInt(i);return r}
 function dp(n,s,k){let states=new Map([[`${n},0,0`,1n]]);for(let f=s;f>=1;f--){const next=new Map();for(const [key,w] of states){const [r,t,sum]=key.split(",").map(Number);for(let c=0;c<=r;c++){if(f===1&&c!==r)continue;const take=Math.min(k-t,c);acc(next,[r-c,t+take,sum+take*f].join(","),w*C(r,c))}}states=next}const out=new Map();for(const [key,w] of states){const [r,t,sum]=key.split(",").map(Number);if(r||t!==k)throw Error("terminal");acc(out,sum,w)}return out}
 function brute(n,s,k){const m=new Map();function rec(a){if(a.length===n){acc(m,[...a].sort((x,y)=>y-x).slice(0,k).reduce((x,y)=>x+y,0),1n);return}for(let v=1;v<=s;v++)rec([...a,v])}rec([]);return m}
 let bins=0;function compare(a,b,total,name){if(a.size!==b.size)throw Error(name+" support");for(const [v,w] of a){if(w!==b.get(v))throw Error(name+" bin");bins++}if([...a.values()].reduce((x,y)=>x+y,0n)!==total)throw Error(name+" mass")}
 const pools=[];for(const [n,s,k] of [[2,20,1],[4,6,3],[5,6,2],[5,10,3],[3,8,1],[3,6,3],[3,6,0]]){const a=dp(n,s,k);compare(a,brute(n,s,k),pow(s,n),"pool");pools.push({n,sides:s,keep:k,ordered:String(pow(s,n)),bins:a.size})}
 function deck(N,K,d){const m=new Map();for(let x=0;x<=d;x++){const w=C(K,x)*C(N-K,d-x);if(w)acc(m,x,w)}return m}
 function deckEnum(N,K,d){const m=new Map();function rec(i,left,h){if(!left){acc(m,h,1n);return}for(let j=i;j<=N-left;j++)rec(j+1,left-1,h+(j<K?1:0))}rec(0,d,0);return m}
 const decks=[];for(const [N,K,d] of [[6,2,2],[10,4,3],[12,6,5],[8,0,3],[8,8,3],[8,4,0]]){const a=deck(N,K,d);compare(a,deckEnum(N,K,d),C(N,d),"deck");decks.push({N,K,draws:d,bins:a.size,ways:String(C(N,d))})}
 function explode(s,depth){const m=new Map();for(let i=0;i<=depth;i++)for(let v=1;v<=s;v++){if(v===s&&i<depth)continue;acc(m,i*s+v,pow(s,depth-i))}return m}
 function explodeEnum(s,d){const m=new Map();function rec(a){if(a.length===d+1){let sum=0;for(const v of a){sum+=v;if(v!==s)break}acc(m,sum,1n);return}for(let v=1;v<=s;v++)rec([...a,v])}rec([]);return m}
 const explosions=[];for(const [s,d] of [[4,0],[4,2],[6,1],[6,3]]){const a=explode(s,d);compare(a,explodeEnum(s,d),pow(s,d+1),"explode");explosions.push({sides:s,max_extra:d,bins:a.size,uncapped_tail:fmt(rat(1,pow(s,d+1)))})}
 const uniform=[];for(let n=2;n<=15;n++){const limit=Math.floor(16/n)*n,counts=Array(n).fill(0);for(let v=0;v<limit;v++)counts[v%n]++;if(!counts.every(x=>x===counts[0]))throw Error("uniform");uniform.push({n,accepted:limit,rejected:16-limit,perOutcome:counts[0]})}
 return {assertions:checks.length,checks,exactBinsCompared:bins,pools,decks,explosions,uniformMappings:uniform,limitations:"Finite synthetic JS model; no general compiler, checked production bit budgets, affine units, correlated arbitrary pools, PRNG implementation or performance claims."};
}
console.log(JSON.stringify(study(),null,2));
