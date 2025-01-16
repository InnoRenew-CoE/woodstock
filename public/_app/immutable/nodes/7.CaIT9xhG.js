import{a as c,t as m,e as R,b as at}from"../chunks/disclose-version.BLKqk4_M.js";import{y as Ee,J as st,x as ot,aN as it,af as nt,G as lt,R as ct,Q as vt,N as dt,ar as ut,p as Se,c as o,b as g,r as a,t as F,g as e,a as qe,d as y,n as Q,ag as M,s as ve,h as Ce,e as D}from"../chunks/runtime.Bbhj79WZ.js";import{s as z}from"../chunks/render.CsQGi6QN.js";import{b as B,p as I,i as q,s as Pe,a as xe,m as He,c as je}from"../chunks/props.DaNf8x0X.js";import{e as we,i as ke}from"../chunks/each.BW8mpJzc.js";import{d as Te}from"../chunks/events.B2JyXTHl.js";import{s as X}from"../chunks/class.2hNHEwuV.js";import{t as Fe,s as Ge,f as ft}from"../chunks/index.-76dDqar.js";import{b as gt}from"../chunks/window.Diu6paOF.js";import{r as pt,b as _t,c as mt,M as G}from"../chunks/MaskedIcon.jNv910dx.js";import"../chunks/legacy.CSot2tHa.js";import{P as Ve}from"../chunks/public.VJoidTgG.js";import{w as Ue}from"../chunks/index.BA7-MmP5.js";import{p as bt}from"../chunks/notifications.D_PBvEQX.js";import{b as ht,a as yt}from"../chunks/input.rRECJIJn.js";import{o as xt}from"../chunks/index-client.C-epJyA0.js";function wt(s,t,n){Ee&&st();var u=s,r=ut,i,b=ot()?it:nt;lt(()=>{b(r,r=t())&&(i&&ct(i),i=vt(()=>n(u)))}),Ee&&(u=dt)}var kt=m('<div class="flex-1"></div>');function St(s){var t=kt();c(s,t)}var ye=(s=>(s[s.Text=0]="Text",s[s.Select=1]="Select",s[s.MultiSelect=2]="MultiSelect",s[s.Tags=3]="Tags",s))(ye||{});const de=Ue(),We=Ue([]),Ye=Ue([]);async function qt(){const s=await fetch(`${Ve}/api/questions`),{questions:t,available_tags:n}=await s.json();We.set(t.toSorted((u,r)=>u.id-r.id)),Ye.set(n)}async function zt(s,t){for(let n of Array.from(s)){const u=t.find(i=>i.file===n.name);if(!u){console.error(`Unable to find answers for file ${n.name}`);continue}const r=new FormData;r.append("answers",JSON.stringify(u.answers)),r.append("file",n,n.name),await fetch(`${Ve}/api/answers`,{method:"POST",body:r})}bt({title:"Success",body:"Submission successful."})}var At=(s,t)=>t(s),Nt=m('<label><input class="hidden" type="checkbox"> </label>');function $t(s,t){Se(t,!0);let n=B(t,"group",31,()=>I([]));B(t,"class",3,"");let u=B(t,"multiple",3,!0);const r=()=>typeof t.value=="number"?+t.value:t.value;let i=y(()=>n().includes(r()));function b(H){const A=r();e(i)?n(n().filter(j=>j!==A)):n().includes(A)||(u()?n([...n(),A]):n([A]))}var f=Nt(),p=o(f);pt(p),p.__change=[At,b];var l=g(p);a(f),F(()=>{X(f,`select-none cursor-pointer px-4 py-2 rounded border ${(e(i)?"text-accent bg-accent/5 border-accent":"border-gray-200 bg-gray-100 opacity-70 hover:opacity-100")??""} `),_t(p,t.value),z(l,` ${t.label??""}`)}),c(s,f),qe()}Te(["change"]);var Mt=m(' <input class="border rounded-lg px-2 py-1 w-full" placeholder="Please type here...">',1);function It(s,t){Q();var n=Mt(),u=M(n);F(()=>z(u,`${JSON.stringify(t.options)??""} `)),Q(),c(s,n)}var Ft=m('<textarea placeholder="Answer here..." class="p-5 min-h-[100px] resize-none w-full rounded border"></textarea>'),jt=m('<div><div class="text-lg font-semibold"> </div> <div> </div> <div class="mt-5 flex gap-5 items-center justify-center flex-wrap"><!></div></div>');function Ct(s,t){Se(t,!0);const[n,u]=Pe(),r=()=>xe(Ye,"$tagsStore",n);let i=B(t,"answer",15),b=B(t,"proceed",15,!1),f=ve(I(i().selection)),p=ve(I(i().text));Ce(()=>{var d;i(i().selection=e(f),!0),i(i().text=e(p),!0),b(e(f).length>0||(((d=e(p))==null?void 0:d.length)??0)>0)});var l=R(),H=M(l);{var A=d=>{var x=jt(),V=o(x),W=o(V,!0);a(V);var Y=g(V,2),v=o(Y,!0);a(Y);var w=g(Y,2),C=o(w);{var P=N=>{var $=Ft();mt($),ht($,()=>e(p),se=>D(p,se)),c(N,$)},ee=N=>{var $=R(),se=M($);{var ze=J=>{It(J,{get options(){return r()}})},ue=J=>{var oe=R(),De=M(oe);we(De,17,()=>t.question.possible_answers,ke,(ie,fe)=>{var Ae=y(()=>t.question.question_type===ye.MultiSelect);$t(ie,{get label(){return e(fe).value},get value(){return e(fe).id},get multiple(){return e(Ae)},get group(){return e(f)},set group(ge){D(f,I(ge))}})}),c(J,oe)};q(se,J=>{t.question.question_type===ye.Tags?J(ze):J(ue,!1)},!0)}c(N,$)};q(C,N=>{t.question.question_type===ye.Text?N(P):N(ee,!1)})}a(w),a(x),F(()=>{z(W,t.question.title),z(v,t.question.text)}),c(d,x)},j=d=>{var x=at();F(()=>z(x,b(!0))),c(d,x)};q(H,d=>{i()?d(A):d(j,!1)})}c(s,l),qe(),u()}var Pt=m('<div class="flex items-center justify-center p-5"><label for="file-upload" class="flex items-center rounded border border-secondary overflow-hidden group bg-secondary/10 hover:bg-secondary cursor-pointer hover:text-white"><div class="bg-secondary p-2 group-hover:bg-white"><!></div> <div class="text-center px-5">Select files</div></label> <input id="file-upload" class="hidden" type="file" multiple></div>');function Tt(s,t){let n=B(t,"files",15);var u=Pt(),r=o(u),i=o(r),b=o(i);G(b,{src:"../sticker-circle.svg",class:"size-4 bg-white group-hover:bg-secondary"}),a(i),Q(2),a(r);var f=g(r,2);a(u),yt(f,n),c(s,u)}var Ut=m('<li class="flex items-center gap-3"><!> </li>'),Dt=m('<div class="font-light pb-2 text-secondary"> </div> <ul class="space-y-2 bg-secondary/5 border border-secondary/50 shadow shadow-secondary/10 py-3 px-5 rounded-lg"></ul>',1),Rt=m('<div class="text-center font-light opacity-80 italic">No files selected</div>'),Bt=m(`<div class="select-none"><p class="text-xl font-semibold">File Upload</p> <div class="py-2 opacity-60">Thank you for choosing to contribute in our research project. Please select files that you'd like to contribute to our system.</div> <div class="py-1 opacity-80">Help us improve our search by contributing. Contribution is a few-step process where you answer some questions about your files.</div> <div><div><!></div> <!></div></div>`);function Jt(s,t){Se(t,!0);const[n,u]=Pe(),r=()=>xe(de,"$filesStore",n);let i=B(t,"proceed",15);Ce(()=>{var d;i((((d=r())==null?void 0:d.length)??0)>0)});var b=Bt(),f=g(o(b),6),p=o(f),l=o(p);Tt(l,{get files(){return He(),r()},set files(d){je(de,I(d))}}),a(p);var H=g(p,2);{var A=d=>{var x=Dt();const V=y(()=>Array.from(r()??[]).map(w=>w.name));var W=M(x),Y=o(W);a(W);var v=g(W,2);we(v,21,()=>e(V),ke,(w,C)=>{var P=Ut(),ee=o(P);G(ee,{src:"../checkmark-circle.svg",class:"opacity-50 size-2.5 bg-primary"});var N=g(ee);a(P),F(()=>z(N,` ${e(C)??""}`)),c(w,P)}),a(v),F(()=>{var w;return z(Y,`Selected files (${((w=r())==null?void 0:w.length)??0??""}):`)}),c(d,x)},j=d=>{var x=Rt();c(d,x)};q(H,d=>{var x;(((x=r())==null?void 0:x.length)??0)>0?d(A):d(j,!1)})}a(f),a(b),c(s,b),qe(),u()}function Lt(s,t,n){t(0),n(void 0)}var Ot=m('<div class="flex justify-center p-5"><div><img src="../tree_of_knowledge.svg" alt="Tree of knowledge" class="size-36 my-5 mx-auto rounded-full bg-secondary/5 p-5 shadow-lg shadow-secondary/30 border border-secondary"> <div class="text-lg font-bold">Submission</div> <div>Thank you for answering our questions. Your files and answers have been safely stored.</div> <button class="my-3 flex items-center gap-2 justify-center w-full py-5"><!> Return to file upload <b class="text-secondary">here</b> or wait 15s for auto-redirection.</button></div></div>');function Et(s,t){let n=B(t,"step",15),u=B(t,"files",15);var r=Ot(),i=o(r),b=g(o(i),6);b.__click=[Lt,n,u];var f=o(b);G(f,{src:"../chevron-right.svg",class:"size-3 bg-secondary"}),Q(3),a(b),a(i),a(r),c(s,r)}Te(["click"]);var Gt=m('<div class="font-nunito opacity-30 pt-2 text-xs uppercase">Files</div>'),Ht=m("<div><li><div><!> </div></li></div>"),Vt=m("<div><div><!> </div> <!></div>"),Wt=m('<div><div class="text-center"><div class="font-bold"> </div> <div><span class="text-accent"> </span> | <span class="opacity-30"> </span></div></div> <!></div>'),Yt=(s,t)=>t(!1),Zt=m('<button class="py-1 px-3 rounded bg-primary text-white opacity-70 hover:opacity-100">Back</button>'),Kt=(s,t)=>t(!0),Xt=m('<button class="py-1 px-3 rounded bg-primary text-white opacity-70 hover:opacity-100 disabled:bg-gray-400">Next</button>'),Qt=m('<span class="text-secondary font-nunito flex items-center gap-2"><!> Required</span>'),er=m('<div class="grid gap-5 grid-cols-[minmax(min-content,300px)_auto] h-full"><div class="select-none bg-dark-background border p-3 rounded-lg"><p class="text-xs opacity-40 uppercase pb-2"> </p> <ul class="p-5 space-y-3"><li><!> File Selection</li> <!> <!> <li><!> Submission</li></ul></div> <div class="bg-dark-background border rounded-lg p-5 grid grid-rows-[auto_min-content]"><!> <div class="flex justify-between gap-5 py-5"><!> <!> <!></div></div></div>'),tr=m('<p class="text-accent text-center">Contribution is not supported on mobile devices due to poor user experience.</p>');function br(s,t){Se(t,!0);const[n,u]=Pe(),r=()=>xe(We,"$questionsStore",n),i=()=>xe(de,"$filesStore",n);xt(async()=>{await qt()});let b=ve(0),f=ve(!1),p=y(()=>{var v;return 1+r().length*(((v=i())==null?void 0:v.length)??1)}),l=ve(0),H=y(()=>Array.from(i()??[]).flatMap(v=>{const w=r().map(C=>({question_id:C.id,selection:[]}));return{file:v.name,answers:w}})),A=y(()=>e(H)[Math.floor((e(l)-1)/r().length)]),j=y(()=>{var v;return(v=e(A))==null?void 0:v.answers[(e(l)-1)%r().length]});function d(v){i()&&(D(l,I(Math.min(Math.max(e(l)+(v?1:-1),0),1+r().length*i().length))),D(f,!1))}Ce(()=>{if(e(l)===e(p)){const v=i();v&&v.length>0&&zt(v,e(H)),setTimeout(()=>{je(de,void 0),D(l,0)},15e3)}});var x=R(),V=M(x);{var W=v=>{var w=er(),C=o(w),P=o(C),ee=o(P);a(P);var N=g(P,2),$=o(N),se=o($),ze=y(()=>`../${(e(l)>=1?"checkmark.svg":"circle.svg")??""}`);G(se,{get src(){return e(ze)},class:"size-2.5 bg-secondary"}),Q(),a($);var ue=g($,2);{var J=_=>{var k=Gt();c(_,k)};q(ue,_=>{var k;(((k=i())==null?void 0:k.length)??0)>0&&_(J)})}var oe=g(ue,2);we(oe,1,()=>Array.from(i()??[]),ke,(_,k,T)=>{var L=Vt();const Z=y(()=>e(l)>=T*r().length+1&&e(l)<(T+1)*r().length+1),h=y(()=>e(l)>=(T+1)*r().length+1),S=y(()=>e(h)?"bg-lime-400":"bg-secondary");var O=o(L),pe=o(O),ne=y(()=>`../${(e(Z)?"chevron-down.svg":e(h)?"checkmark.svg":"circle.svg")??""}`),_e=y(()=>`${(e(h)||e(Z)?"size-3":"size-2")??""} ${e(S)??""}`);G(pe,{get src(){return e(ne)},get class(){return e(_e)}});var Ne=g(pe);a(O);var $e=g(O,2);we($e,1,r,ke,(U,le,te)=>{var be=R();const re=y(()=>e(l)==1+te+T*r().length),ae=y(()=>e(l)>1+te+T*r().length);var Me=M(be);{var he=ce=>{var K=Ht(),E=o(K),Ie=o(E),Oe=o(Ie),tt=y(()=>`../${(e(ae)?"checkmark.svg":e(re)?"chevron-right.svg":"circle.svg")??""}`);G(Oe,{get src(){return e(tt)},class:"w-3 h-3 bg-secondary"});var rt=g(Oe);a(Ie),a(E),a(K),F(()=>{X(E,`pl-5 ${(e(ae)||e(re)?"":"opacity-30")??""}`),X(Ie,`flex gap-3 items-center ${(e(re)?"font-bold":"")??""}`),z(rt,` ${e(le).title??""}`)}),Fe(1,K,()=>Ge),Fe(2,K,()=>Ge),c(ce,K)};q(Me,ce=>{e(Z)&&ce(he)})}c(U,be)}),a(L),F(()=>{X(L,`px-3 py-1 shadow-sm bg-secondary/5 border rounded-lg ${(e(h)?"border-lime-400 bg-lime-400/10":"border-secondary/30")??""}`),X(O,`flex items-center gap-2 ${(e(h)?"text-lime-500":"")??""}`),z(Ne,` ${e(k).name??""}`)}),c(_,L)});var ie=g(oe,2),fe=o(ie),Ae=y(()=>`../${(e(l)===e(p)?"checkmark.svg":"circle.svg")??""}`);G(fe,{get src(){return e(Ae)},class:"size-2.5 bg-secondary"}),Q(),a(ie),a(N),a(C);var ge=g(C,2),Re=o(ge);{var Ze=_=>{Jt(_,{get proceed(){return e(f)},set proceed(k){D(f,I(k))}})},Ke=_=>{var k=R(),T=M(k);{var L=h=>{Et(h,{get step(){return e(l)},set step(S){D(l,I(S))},get files(){return He(),i()},set files(S){je(de,I(S))}})},Z=h=>{var S=R(),O=M(S);{var pe=ne=>{var _e=R();const Ne=y(()=>r().filter(me=>{var U;return me.id===((U=e(j))==null?void 0:U.question_id)})[0]);var $e=M(_e);wt($e,()=>e(l),me=>{var U=Wt(),le=o(U),te=o(le),be=o(te,!0);a(te);var re=g(te,2),ae=o(re),Me=o(ae,!0);a(ae);var he=g(ae,2),ce=o(he,!0);a(he),a(re),a(le);var K=g(le,2);Ct(K,{get question(){return e(Ne)},get answer(){return e(j)},get proceed(){return e(f)},set proceed(E){D(f,I(E))}}),a(U),F(()=>{var E;z(be,(E=e(A))==null?void 0:E.file),z(Me,(e(l)-1)%r().length+1),z(ce,r().length)}),Fe(1,U,()=>ft),c(me,U)}),c(ne,_e)};q(O,ne=>{e(j)&&ne(pe)},!0)}c(h,S)};q(T,h=>{e(l)===e(p)?h(L):h(Z,!1)},!0)}c(_,k)};q(Re,_=>{e(l)===0?_(Ze):_(Ke,!1)})}var Be=g(Re,2),Je=o(Be);{var Xe=_=>{var k=Zt();k.__click=[Yt,d],c(_,k)};q(Je,_=>{e(l)>=1&&e(l)!==e(p)&&_(Xe)})}var Le=g(Je,2);St(Le);var Qe=g(Le,2);{var et=_=>{var k=R(),T=M(k);{var L=h=>{var S=Xt();S.__click=[Kt,d],c(h,S)},Z=h=>{var S=Qt(),O=o(S);G(O,{src:"../chevron-right.svg",class:"size-3 bg-secondary animate-pulse"}),Q(),a(S),c(h,S)};q(T,h=>{e(f)?h(L):h(Z,!1)})}c(_,k)};q(Qe,_=>{e(l)<e(p)&&_(et)})}a(Be),a(ge),a(w),F(()=>{z(ee,`Step ${e(l)??""} / ${e(p)??""}`),X($,`flex items-center gap-3 ${(e(l)===0?"font-semibold":"")??""}`),X(ie,`flex items-center gap-3 ${(e(l)===e(p)?"font-bold":"opacity-45")??""}`)}),c(v,w)},Y=v=>{var w=tr();c(v,w)};q(V,v=>{e(b)>640?v(W):v(Y,!1)})}gt("innerWidth",v=>D(b,I(v))),c(s,x),qe(),u()}Te(["click"]);export{br as component};
