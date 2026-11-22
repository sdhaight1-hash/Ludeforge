// Original finite experiment comparison. Run: node research/prototypes/comparison-check.mjs
function pairedStudy(){
 const assert=(b,m)=>{if(!b)throw Error(m)};
 // Original finite fixture. A threshold 12, B threshold 10. Same d20 draw only if roles match.
 const a=r=>r>=12?1:0,b=r=>r>=10?1:0;
 function summarize(rows){let n=0,sum=0,sq=0,sa=0,sb=0;for(const [x,y] of rows){n++;const d=y-x;sum+=d;sq+=d*d;sa+=x;sb+=y}return {n,sumDelta:sum,sumSquaredDelta:sq,successA:sa,successB:sb,meanDelta:sum/n,populationVariance:sq/n-(sum/n)**2}}
 const paired=Array.from({length:20},(_,i)=>[a(i+1),b(i+1)]);
 const independent=Array.from({length:400},(_,i)=>[a(Math.floor(i/20)+1),b(i%20+1)]);
 const p=summarize(paired),q=summarize(independent);
 assert(p.sumDelta===2&&q.sumDelta===40,"expected difference");
 assert(Math.abs(p.populationVariance-0.09)<1e-12&&Math.abs(q.populationVariance-0.495)<1e-12,"variance");
 // Equal marginal success at reversed pairing but negative covariance increases variance.
 const opposite=Array.from({length:20},(_,i)=>[a(i+1),b(20-i)]);
 const o=summarize(opposite);assert(o.successA===p.successA&&o.successB===p.successB&&o.populationVariance>q.populationVariance,"negative covariance");
 // Exact integer sufficient statistics make deterministic reductions for any traversal/partition order.
 const records=independent.map(([x,y],id)=>({id,x,y}));
 const reduce=rs=>rs.reduce((s,r)=>{const d=r.y-r.x;return [s[0]+1,s[1]+d,s[2]+d*d,s[3]+r.x,s[4]+r.y]},[0,0,0,0,0]);
 const baseline=JSON.stringify(reduce(records));let partitionCases=0;
 for(const workers of [1,2,3,7,16]){const groups=Array.from({length:workers},(_,i)=>records.filter(r=>r.id%workers===i));for(const order of [groups,[...groups].reverse()]){const total=order.map(g=>reduce([...g].reverse())).reduce((a,b)=>a.map((x,i)=>x+b[i]),[0,0,0,0,0]);assert(JSON.stringify(total)===baseline,"reduction order");partitionCases++}}
 // Show mismatch from naïve positional pairing if a variant requests an incidental draw.
 const receipt={"weather":2,"attack":15,"detail":3};
 function evaluate(schedule){return schedule.map(role=>({role,result:receipt[role]}))}
 const baselineTrace=evaluate(["weather","attack"]),variantTrace=evaluate(["weather","detail","attack"]);
 assert(baselineTrace.find(x=>x.role==="attack").result===variantTrace.find(x=>x.role==="attack").result,"semantic alignment");
 assert(baselineTrace[1].role!==variantTrace[1].role,"positional mismatch");
 // Report all dispositions, not just completed successful trials.
 const dispositions=["completed","completed","rejected","timeout","completed","limit"];
 const counts={};for(const d of dispositions)counts[d]=(counts[d]||0)+1;
 assert(Object.values(counts).reduce((a,b)=>a+b,0)===dispositions.length,"disposition accounting");
 return {paired:p,independent:q,oppositePairing:o,varianceRatioIndependentToPaired:q.populationVariance/p.populationVariance,partitionCases,baselineTrace,variantTrace,dispositions:counts,limitations:"Exhaustive uniform finite outcome comparison, not a statistical sampling run. Population variances are computed over enumerated outcomes; variance ratio predicts comparisons of equal numbers of IID pairs under those coupling assumptions, not measured speedup. Partitioned reductions simulated sequentially, not real concurrency. Semantic receipt is fixed, not a PRNG or complete replay engine."};
}
console.log(JSON.stringify(pairedStudy(),null,2));
