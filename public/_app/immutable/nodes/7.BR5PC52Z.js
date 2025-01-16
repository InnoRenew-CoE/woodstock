import{a as c,t as m,e as U,b as at}from"../chunks/disclose-version.xnC7mC_x.js";import{j as Ee,I as st,D as ot,aO as it,aa as nt,f as lt,O as ct,h as vt,k as dt,ah as ut,p as Se,c as s,b as p,r as a,t as j,g as e,a as qe,d as y,l as Q,ab as N,s as ve,o as Ce,e as T}from"../chunks/runtime.BWQzXbsj.js";import{d as De,s as z}from"../chunks/render.Dk_7IlEG.js";import{a as B,p as I,i as q}from"../chunks/props.B92kTshy.js";import{e as xe,i as we}from"../chunks/each.DJz7YfYu.js";import{s as X}from"../chunks/class.CY0ydPG4.js";import{t as je,s as He,f as ft}from"../chunks/index.BWIZlTwn.js";import{b as pt}from"../chunks/window.l_Scip2-.js";import{s as Oe,a as ke,m as Ve,b as Fe}from"../chunks/store.Bzbw3R17.js";import{r as gt,a as _t,M as H}from"../chunks/MaskedIcon.nnt84fH3.js";import"../chunks/legacy.CctD85KV.js";import{P as We}from"../chunks/public.C-hhuxPZ.js";import{w as Pe}from"../chunks/index.DEYB36my.js";import{p as mt}from"../chunks/notifications.DrIwINQU.js";import{r as bt}from"../chunks/shared.D7kxSIzK.js";import{b as ht,a as yt}from"../chunks/input.CIl_CBti.js";import{o as xt}from"../chunks/index-client.DO-FP4f5.js";function wt(o,t,n){Ee&&st();var u=o,r=ut,i,b=ot()?it:nt;lt(()=>{b(r,r=t())&&(i&&ct(i),i=vt(()=>n(u)))}),Ee&&(u=dt)}var kt=m('<div class="flex-1"></div>');function St(o){var t=kt();c(o,t)}var ye=(o=>(o[o.Text=0]="Text",o[o.Select=1]="Select",o[o.MultiSelect=2]="MultiSelect",o[o.Tags=3]="Tags",o))(ye||{});const de=Pe(),Ye=Pe([]),Ze=Pe([]);async function qt(){const t=await(await fetch(`${We}/api/questions`)).json(),{questions:n,available_tags:u}=JSON.parse(t);Ye.set(n.toSorted((r,i)=>r.id-i.id)),Ze.set(u)}async function zt(o,t){for(let n of Array.from(o)){const u=t.find(i=>i.file===n.name);if(!u){console.error(`Unable to find answers for file ${n.name}`);continue}const r=new FormData;r.append("answers",JSON.stringify(u.answers)),r.append("file",n,n.name),await fetch(`${We}/api/answers`,{method:"POST",body:r})}mt({title:"Success",body:"Submission successful."})}var At=(o,t)=>t(o),$t=m('<label><input class="hidden" type="checkbox"> </label>');function Mt(o,t){Se(t,!0);let n=B(t,"group",31,()=>I([]));B(t,"class",3,"");let u=B(t,"multiple",3,!0);const r=()=>typeof t.value=="number"?+t.value:t.value;let i=y(()=>n().includes(r()));function b(V){const A=r();e(i)?n(n().filter(F=>F!==A)):n().includes(A)||(u()?n([...n(),A]):n([A]))}var f=$t(),g=s(f);gt(g),g.__change=[At,b];var l=p(g);a(f),j(()=>{X(f,`select-none cursor-pointer px-4 py-2 rounded border ${(e(i)?"text-accent bg-accent/5 border-accent":"border-gray-200 bg-gray-100 opacity-70 hover:opacity-100")??""} `),_t(g,t.value),z(l,` ${t.label??""}`)}),c(o,f),qe()}De(["change"]);var Nt=m(' <input class="border rounded-lg px-2 py-1 w-full" placeholder="Please type here...">',1);function It(o,t){Q();var n=Nt(),u=N(n);j(()=>z(u,`${JSON.stringify(t.options)??""} `)),Q(),c(o,n)}var jt=m('<textarea placeholder="Answer here..." class="p-5 min-h-[100px] resize-none w-full rounded border"></textarea>'),Ft=m('<div><div class="text-lg font-semibold"> </div> <div> </div> <div class="mt-5 flex gap-5 items-center justify-center flex-wrap"><!></div></div>');function Ct(o,t){Se(t,!0);const[n,u]=Oe(),r=()=>ke(Ze,"$tagsStore",n);let i=B(t,"answer",15),b=B(t,"proceed",15,!1),f=ve(I(i().selection)),g=ve(I(i().text));Ce(()=>{var d;i(i().selection=e(f),!0),i(i().text=e(g),!0),b(e(f).length>0||(((d=e(g))==null?void 0:d.length)??0)>0)});var l=U(),V=N(l);{var A=d=>{var x=Ft(),W=s(x),Y=s(W,!0);a(W);var Z=p(W,2),v=s(Z,!0);a(Z);var w=p(Z,2),C=s(w);{var D=$=>{var M=jt();bt(M),ht(M,()=>e(g),se=>T(g,se)),c($,M)},ee=$=>{var M=U(),se=N(M);{var ze=J=>{It(J,{get options(){return r()}})},ue=J=>{var oe=U(),Te=N(oe);xe(Te,17,()=>t.question.possible_answers,we,(ie,fe)=>{var Ae=y(()=>t.question.question_type===ye.MultiSelect);Mt(ie,{get label(){return e(fe).value},get value(){return e(fe).id},get multiple(){return e(Ae)},get group(){return e(f)},set group(pe){T(f,I(pe))}})}),c(J,oe)};q(se,J=>{t.question.question_type===ye.Tags?J(ze):J(ue,!1)},!0)}c($,M)};q(C,$=>{t.question.question_type===ye.Text?$(D):$(ee,!1)})}a(w),a(x),j(()=>{z(Y,t.question.title),z(v,t.question.text)}),c(d,x)},F=d=>{var x=at();j(()=>z(x,b(!0))),c(d,x)};q(V,d=>{i()?d(A):d(F,!1)})}c(o,l),qe(),u()}var Dt=m('<div class="flex items-center justify-center p-5"><label for="file-upload" class="flex items-center rounded border border-secondary overflow-hidden group bg-secondary/10 hover:bg-secondary cursor-pointer hover:text-white"><div class="bg-secondary p-2 group-hover:bg-white"><!></div> <div class="text-center px-5">Select files</div></label> <input id="file-upload" class="hidden" type="file" multiple></div>');function Ot(o,t){let n=B(t,"files",15);var u=Dt(),r=s(u),i=s(r),b=s(i);H(b,{src:"../sticker-circle.svg",class:"size-4 bg-white group-hover:bg-secondary"}),a(i),Q(2),a(r);var f=p(r,2);a(u),yt(f,n),c(o,u)}var Pt=m('<li class="flex items-center gap-3"><!> </li>'),Tt=m('<div class="font-light pb-2 text-secondary"> </div> <ul class="space-y-2 bg-secondary/5 border border-secondary/50 shadow shadow-secondary/10 py-3 px-5 rounded-lg"></ul>',1),Ut=m('<div class="text-center font-light opacity-80 italic">No files selected</div>'),Bt=m(`<div class="select-none"><p class="text-xl font-semibold">File Upload</p> <div class="py-2 opacity-60">Thank you for choosing to contribute in our research project. Please select files that you'd like to contribute to our system.</div> <div class="py-1 opacity-80">Help us improve our search by contributing. Contribution is a few-step process where you answer some questions about your files.</div> <div><div><!></div> <!></div></div>`);function Jt(o,t){Se(t,!0);const[n,u]=Oe(),r=()=>ke(de,"$filesStore",n);let i=B(t,"proceed",15);Ce(()=>{var d;i((((d=r())==null?void 0:d.length)??0)>0)});var b=Bt(),f=p(s(b),6),g=s(f),l=s(g);Ot(l,{get files(){return Ve(),r()},set files(d){Fe(de,I(d))}}),a(g);var V=p(g,2);{var A=d=>{var x=Tt();const W=y(()=>Array.from(r()??[]).map(w=>w.name));var Y=N(x),Z=s(Y);a(Y);var v=p(Y,2);xe(v,21,()=>e(W),we,(w,C)=>{var D=Pt(),ee=s(D);H(ee,{src:"../checkmark-circle.svg",class:"opacity-50 size-2.5 bg-primary"});var $=p(ee);a(D),j(()=>z($,` ${e(C)??""}`)),c(w,D)}),a(v),j(()=>{var w;return z(Z,`Selected files (${((w=r())==null?void 0:w.length)??0??""}):`)}),c(d,x)},F=d=>{var x=Ut();c(d,x)};q(V,d=>{var x;(((x=r())==null?void 0:x.length)??0)>0?d(A):d(F,!1)})}a(f),a(b),c(o,b),qe(),u()}function Lt(o,t,n){t(0),n(void 0)}var Rt=m('<div class="flex justify-center p-5"><div><img src="../tree_of_knowledge.svg" alt="Tree of knowledge" class="size-36 my-5 mx-auto rounded-full bg-secondary/5 p-5 shadow-lg shadow-secondary/30 border border-secondary"> <div class="text-lg font-bold">Submission</div> <div>Thank you for answering our questions. Your files and answers have been safely stored.</div> <button class="my-3 flex items-center gap-2 justify-center w-full py-5"><!> Return to file upload <b class="text-secondary">here</b> or wait 15s for auto-redirection.</button></div></div>');function Et(o,t){let n=B(t,"step",15),u=B(t,"files",15);var r=Rt(),i=s(r),b=p(s(i),6);b.__click=[Lt,n,u];var f=s(b);H(f,{src:"../chevron-right.svg",class:"size-3 bg-secondary"}),Q(3),a(b),a(i),a(r),c(o,r)}De(["click"]);var Ht=m('<div class="font-nunito opacity-30 pt-2 text-xs uppercase">Files</div>'),Vt=m("<div><li><div><!> </div></li></div>"),Wt=m("<div><div><!> </div> <!></div>"),Yt=m('<div><div class="text-center"><div class="font-bold"> </div> <div><span class="text-accent"> </span> | <span class="opacity-30"> </span></div></div> <!></div>'),Zt=(o,t)=>t(!1),Gt=m('<button class="py-1 px-3 rounded bg-primary text-white opacity-70 hover:opacity-100">Back</button>'),Kt=(o,t)=>t(!0),Xt=m('<button class="py-1 px-3 rounded bg-primary text-white opacity-70 hover:opacity-100 disabled:bg-gray-400">Next</button>'),Qt=m('<span class="text-secondary font-nunito flex items-center gap-2"><!> Required</span>'),er=m('<div class="grid gap-5 grid-cols-[minmax(min-content,300px)_auto] h-full"><div class="select-none bg-dark-background border p-3 rounded-lg"><p class="text-xs opacity-40 uppercase pb-2"> </p> <ul class="p-5 space-y-3"><li><!> File Selection</li> <!> <!> <li><!> Submission</li></ul></div> <div class="bg-dark-background border rounded-lg p-5 grid grid-rows-[auto_min-content]"><!> <div class="flex justify-between gap-5 py-5"><!> <!> <!></div></div></div>'),tr=m('<p class="text-accent text-center">Contribution is not supported on mobile devices due to poor user experience.</p>');function hr(o,t){Se(t,!0);const[n,u]=Oe(),r=()=>ke(Ye,"$questionsStore",n),i=()=>ke(de,"$filesStore",n);xt(async()=>{await qt()});let b=ve(0),f=ve(!1),g=y(()=>{var v;return 1+r().length*(((v=i())==null?void 0:v.length)??1)}),l=ve(0),V=y(()=>Array.from(i()??[]).flatMap(v=>{const w=r().map(C=>({question_id:C.id,selection:[]}));return{file:v.name,answers:w}})),A=y(()=>e(V)[Math.floor((e(l)-1)/r().length)]),F=y(()=>{var v;return(v=e(A))==null?void 0:v.answers[(e(l)-1)%r().length]});function d(v){i()&&(T(l,I(Math.min(Math.max(e(l)+(v?1:-1),0),1+r().length*i().length))),T(f,!1))}Ce(()=>{if(e(l)===e(g)){const v=i();v&&v.length>0&&zt(v,e(V)),setTimeout(()=>{Fe(de,void 0),T(l,0)},15e3)}});var x=U(),W=N(x);{var Y=v=>{var w=er(),C=s(w),D=s(C),ee=s(D);a(D);var $=p(D,2),M=s($),se=s(M),ze=y(()=>`../${(e(l)>=1?"checkmark.svg":"circle.svg")??""}`);H(se,{get src(){return e(ze)},class:"size-2.5 bg-secondary"}),Q(),a(M);var ue=p(M,2);{var J=_=>{var k=Ht();c(_,k)};q(ue,_=>{var k;(((k=i())==null?void 0:k.length)??0)>0&&_(J)})}var oe=p(ue,2);xe(oe,1,()=>Array.from(i()??[]),we,(_,k,O)=>{var L=Wt();const G=y(()=>e(l)>=O*r().length+1&&e(l)<(O+1)*r().length+1),h=y(()=>e(l)>=(O+1)*r().length+1),S=y(()=>e(h)?"bg-lime-400":"bg-secondary");var R=s(L),ge=s(R),ne=y(()=>`../${(e(G)?"chevron-down.svg":e(h)?"checkmark.svg":"circle.svg")??""}`),_e=y(()=>`${(e(h)||e(G)?"size-3":"size-2")??""} ${e(S)??""}`);H(ge,{get src(){return e(ne)},get class(){return e(_e)}});var $e=p(ge);a(R);var Me=p(R,2);xe(Me,1,r,we,(P,le,te)=>{var be=U();const re=y(()=>e(l)==1+te+O*r().length),ae=y(()=>e(l)>1+te+O*r().length);var Ne=N(be);{var he=ce=>{var K=Vt(),E=s(K),Ie=s(E),Re=s(Ie),tt=y(()=>`../${(e(ae)?"checkmark.svg":e(re)?"chevron-right.svg":"circle.svg")??""}`);H(Re,{get src(){return e(tt)},class:"w-3 h-3 bg-secondary"});var rt=p(Re);a(Ie),a(E),a(K),j(()=>{X(E,`pl-5 ${(e(ae)||e(re)?"":"opacity-30")??""}`),X(Ie,`flex gap-3 items-center ${(e(re)?"font-bold":"")??""}`),z(rt,` ${e(le).title??""}`)}),je(1,K,()=>He),je(2,K,()=>He),c(ce,K)};q(Ne,ce=>{e(G)&&ce(he)})}c(P,be)}),a(L),j(()=>{X(L,`px-3 py-1 shadow-sm bg-secondary/5 border rounded-lg ${(e(h)?"border-lime-400 bg-lime-400/10":"border-secondary/30")??""}`),X(R,`flex items-center gap-2 ${(e(h)?"text-lime-500":"")??""}`),z($e,` ${e(k).name??""}`)}),c(_,L)});var ie=p(oe,2),fe=s(ie),Ae=y(()=>`../${(e(l)===e(g)?"checkmark.svg":"circle.svg")??""}`);H(fe,{get src(){return e(Ae)},class:"size-2.5 bg-secondary"}),Q(),a(ie),a($),a(C);var pe=p(C,2),Ue=s(pe);{var Ge=_=>{Jt(_,{get proceed(){return e(f)},set proceed(k){T(f,I(k))}})},Ke=_=>{var k=U(),O=N(k);{var L=h=>{Et(h,{get step(){return e(l)},set step(S){T(l,I(S))},get files(){return Ve(),i()},set files(S){Fe(de,I(S))}})},G=h=>{var S=U(),R=N(S);{var ge=ne=>{var _e=U();const $e=y(()=>r().filter(me=>{var P;return me.id===((P=e(F))==null?void 0:P.question_id)})[0]);var Me=N(_e);wt(Me,()=>e(l),me=>{var P=Yt(),le=s(P),te=s(le),be=s(te,!0);a(te);var re=p(te,2),ae=s(re),Ne=s(ae,!0);a(ae);var he=p(ae,2),ce=s(he,!0);a(he),a(re),a(le);var K=p(le,2);Ct(K,{get question(){return e($e)},get answer(){return e(F)},get proceed(){return e(f)},set proceed(E){T(f,I(E))}}),a(P),j(()=>{var E;z(be,(E=e(A))==null?void 0:E.file),z(Ne,(e(l)-1)%r().length+1),z(ce,r().length)}),je(1,P,()=>ft),c(me,P)}),c(ne,_e)};q(R,ne=>{e(F)&&ne(ge)},!0)}c(h,S)};q(O,h=>{e(l)===e(g)?h(L):h(G,!1)},!0)}c(_,k)};q(Ue,_=>{e(l)===0?_(Ge):_(Ke,!1)})}var Be=p(Ue,2),Je=s(Be);{var Xe=_=>{var k=Gt();k.__click=[Zt,d],c(_,k)};q(Je,_=>{e(l)>=1&&e(l)!==e(g)&&_(Xe)})}var Le=p(Je,2);St(Le);var Qe=p(Le,2);{var et=_=>{var k=U(),O=N(k);{var L=h=>{var S=Xt();S.__click=[Kt,d],c(h,S)},G=h=>{var S=Qt(),R=s(S);H(R,{src:"../chevron-right.svg",class:"size-3 bg-secondary animate-pulse"}),Q(),a(S),c(h,S)};q(O,h=>{e(f)?h(L):h(G,!1)})}c(_,k)};q(Qe,_=>{e(l)<e(g)&&_(et)})}a(Be),a(pe),a(w),j(()=>{z(ee,`Step ${e(l)??""} / ${e(g)??""}`),X(M,`flex items-center gap-3 ${(e(l)===0?"font-semibold":"")??""}`),X(ie,`flex items-center gap-3 ${(e(l)===e(g)?"font-bold":"opacity-45")??""}`)}),c(v,w)},Z=v=>{var w=tr();c(v,w)};q(W,v=>{e(b)>640?v(Y):v(Z,!1)})}pt("innerWidth",v=>T(b,I(v))),c(o,x),qe(),u()}De(["click"]);export{hr as component};
