var qn=Array.isArray,Wt=Array.prototype.indexOf,Pn=Array.from,Fn=Object.defineProperty,pt=Object.getOwnPropertyDescriptor,Xt=Object.getOwnPropertyDescriptors,Ln=Object.prototype,Mn=Array.prototype,Qt=Object.getPrototypeOf;function Yn(t){return typeof t=="function"}const Hn=()=>{};function jn(t){return t()}function wt(t){for(var n=0;n<t.length;n++)t[n]()}const y=2,Tt=4,H=8,ot=16,A=32,W=64,V=128,D=256,G=512,E=1024,O=2048,X=4096,b=8192,q=16384,tn=32768,At=65536,Bn=1<<17,nn=1<<19,mt=1<<20,ht=Symbol("$state"),Un=Symbol("legacy props"),Vn=Symbol("");function It(t){return t===this.v}function rn(t,n){return t!=t?n==n:t!==n||t!==null&&typeof t=="object"||typeof t=="function"}function Gn(t,n){return t!==n}function gt(t){return!rn(t,this.v)}function en(t){throw new Error("https://svelte.dev/e/effect_in_teardown")}function sn(){throw new Error("https://svelte.dev/e/effect_in_unowned_derived")}function ln(t){throw new Error("https://svelte.dev/e/effect_orphan")}function an(){throw new Error("https://svelte.dev/e/effect_update_depth_exceeded")}function Kn(){throw new Error("https://svelte.dev/e/hydration_failed")}function $n(t){throw new Error("https://svelte.dev/e/props_invalid_value")}function Zn(){throw new Error("https://svelte.dev/e/state_descriptors_fixed")}function zn(){throw new Error("https://svelte.dev/e/state_prototype_fixed")}function un(){throw new Error("https://svelte.dev/e/state_unsafe_local_read")}function on(){throw new Error("https://svelte.dev/e/state_unsafe_mutation")}let Q=!1;function Jn(){Q=!0}const Wn=1,Xn=2,Qn=4,tr=8,nr=16,rr=1,er=2,sr=4,lr=8,ar=16,ur=1,or=2,ir=4,fr=1,_r=2,fn="[",_n="[!",cn="]",Rt={},cr=Symbol();function it(t,n){var r={f:0,v:t,reactions:null,equals:It,rv:0,wv:0};return r}function vr(t){return vn(it(t))}function pr(t,n=!1){var e;const r=it(t);return n||(r.equals=gt),Q&&f!==null&&f.l!==null&&((e=f.l).s??(e.s=[])).push(r),r}function vn(t){return o!==null&&o.f&y&&(T===null?On([t]):T.push(t)),t}function hr(t,n){return o!==null&&ct()&&o.f&(y|ot)&&(T===null||!T.includes(t))&&on(),pn(t,n)}function pn(t,n){return t.equals(n)||(t.v,t.v=n,t.wv=Gt(),Ot(t,O),ct()&&u!==null&&u.f&E&&!(u.f&A)&&(h!==null&&h.includes(t)?(m(u,O),rt(u)):I===null?Sn([t]):I.push(t))),n}function Ot(t,n){var r=t.reactions;if(r!==null)for(var e=ct(),s=r.length,l=0;l<s;l++){var a=r[l],i=a.f;i&O||!e&&a===u||(m(a,n),i&(E|D)&&(i&y?Ot(a,X):rt(a)))}}function St(t){console.warn("https://svelte.dev/e/hydration_mismatch")}let S=!1;function dr(t){S=t}let w;function F(t){if(t===null)throw St(),Rt;return w=t}function Er(){return F(N(w))}function yr(t){if(S){if(N(w)!==null)throw St(),Rt;w=t}}function wr(t=1){if(S){for(var n=t,r=w;n--;)r=N(r);w=r}}function Tr(){for(var t=0,n=w;;){if(n.nodeType===8){var r=n.data;if(r===cn){if(t===0)return n;t-=1}else(r===fn||r===_n)&&(t+=1)}var e=N(n);n.remove(),n=e}}var dt,kt,Dt;function Ar(){if(dt===void 0){dt=window;var t=Element.prototype,n=Node.prototype;kt=pt(n,"firstChild").get,Dt=pt(n,"nextSibling").get,t.__click=void 0,t.__className="",t.__attributes=null,t.__styles=null,t.__e=void 0,Text.prototype.__t=void 0}}function et(t=""){return document.createTextNode(t)}function st(t){return kt.call(t)}function N(t){return Dt.call(t)}function mr(t,n){if(!S)return st(t);var r=st(w);if(r===null)r=w.appendChild(et());else if(n&&r.nodeType!==3){var e=et();return r==null||r.before(e),F(e),e}return F(r),r}function Ir(t,n){if(!S){var r=st(t);return r instanceof Comment&&r.data===""?N(r):r}return w}function gr(t,n=1,r=!1){let e=S?w:t;for(var s;n--;)s=e,e=N(e);if(!S)return e;var l=e==null?void 0:e.nodeType;if(r&&l!==3){var a=et();return e===null?s==null||s.after(a):e.before(a),F(a),a}return F(e),e}function Rr(t){t.textContent=""}function hn(t){var n=y|O;u===null?n|=D:u.f|=mt;var r=o!==null&&o.f&y?o:null;const e={children:null,ctx:f,deps:null,equals:It,f:n,fn:t,reactions:null,rv:0,v:null,wv:0,parent:r??u};return r!==null&&(r.children??(r.children=[])).push(e),e}function Or(t){const n=hn(t);return n.equals=gt,n}function Nt(t){var n=t.children;if(n!==null){t.children=null;for(var r=0;r<n.length;r+=1){var e=n[r];e.f&y?ft(e):k(e)}}}function dn(t){for(var n=t.parent;n!==null;){if(!(n.f&y))return n;n=n.parent}return null}function xt(t){var n,r=u;z(dn(t));try{Nt(t),n=Kt(t)}finally{z(r)}return n}function Ct(t){var n=xt(t),r=(R||t.f&D)&&t.deps!==null?X:E;m(t,r),t.equals(n)||(t.v=n,t.wv=Gt())}function ft(t){Nt(t),Y(t,0),m(t,q),t.v=t.children=t.deps=t.ctx=t.reactions=null}function bt(t){u===null&&o===null&&ln(),o!==null&&o.f&D&&sn(),_t&&en()}function En(t,n){var r=n.last;r===null?n.last=n.first=t:(r.next=t,t.prev=r,n.last=t)}function P(t,n,r,e=!0){var s=(t&W)!==0,l=u,a={ctx:f,deps:null,deriveds:null,nodes_start:null,nodes_end:null,f:t|O,first:null,fn:n,last:null,next:null,parent:s?null:l,prev:null,teardown:null,transitions:null,wv:0};if(r){var i=x;try{Et(!0),nt(a),a.f|=tn}catch(c){throw k(a),c}finally{Et(i)}}else n!==null&&rt(a);var _=r&&a.deps===null&&a.first===null&&a.nodes_start===null&&a.teardown===null&&(a.f&(mt|V))===0;if(!_&&!s&&e&&(l!==null&&En(a,l),o!==null&&o.f&y)){var p=o;(p.children??(p.children=[])).push(a)}return a}function Sr(t){const n=P(H,null,!1);return m(n,E),n.teardown=t,n}function kr(t){bt();var n=u!==null&&(u.f&A)!==0&&f!==null&&!f.m;if(n){var r=f;(r.e??(r.e=[])).push({fn:t,effect:u,reaction:o})}else{var e=qt(t);return e}}function Dr(t){return bt(),yn(t)}function Nr(t){const n=P(W,t,!0);return(r={})=>new Promise(e=>{r.outro?An(n,()=>{k(n),e(void 0)}):(k(n),e(void 0))})}function qt(t){return P(Tt,t,!1)}function yn(t){return P(H,t,!0)}function xr(t){return wn(t)}function wn(t,n=0){return P(H|ot|n,t,!0)}function Cr(t,n=!0){return P(H|A,t,!0,n)}function Pt(t){var n=t.teardown;if(n!==null){const r=_t,e=o;yt(!0),Z(null);try{n.call(null)}finally{yt(r),Z(e)}}}function Ft(t){var n=t.deriveds;if(n!==null){t.deriveds=null;for(var r=0;r<n.length;r+=1)ft(n[r])}}function Lt(t,n=!1){var r=t.first;for(t.first=t.last=null;r!==null;){var e=r.next;k(r,n),r=e}}function Tn(t){for(var n=t.first;n!==null;){var r=n.next;n.f&A||k(n),n=r}}function k(t,n=!0){var r=!1;if((n||t.f&nn)&&t.nodes_start!==null){for(var e=t.nodes_start,s=t.nodes_end;e!==null;){var l=e===s?null:N(e);e.remove(),e=l}r=!0}Lt(t,n&&!r),Ft(t),Y(t,0),m(t,q);var a=t.transitions;if(a!==null)for(const _ of a)_.stop();Pt(t);var i=t.parent;i!==null&&i.first!==null&&Mt(t),t.next=t.prev=t.teardown=t.ctx=t.deps=t.fn=t.nodes_start=t.nodes_end=null}function Mt(t){var n=t.parent,r=t.prev,e=t.next;r!==null&&(r.next=e),e!==null&&(e.prev=r),n!==null&&(n.first===t&&(n.first=e),n.last===t&&(n.last=r))}function An(t,n){var r=[];Yt(t,r,!0),mn(r,()=>{k(t),n&&n()})}function mn(t,n){var r=t.length;if(r>0){var e=()=>--r||n();for(var s of t)s.out(e)}else n()}function Yt(t,n,r){if(!(t.f&b)){if(t.f^=b,t.transitions!==null)for(const a of t.transitions)(a.is_global||r)&&n.push(a);for(var e=t.first;e!==null;){var s=e.next,l=(e.f&At)!==0||(e.f&A)!==0;Yt(e,n,l?r:!1),e=s}}}function br(t){Ht(t,!0)}function Ht(t,n){if(t.f&b){j(t)&&nt(t),t.f^=b;for(var r=t.first;r!==null;){var e=r.next,s=(r.f&At)!==0||(r.f&A)!==0;Ht(r,s?n:!1),r=e}if(t.transitions!==null)for(const l of t.transitions)(l.is_global||n)&&l.in()}}const In=typeof requestIdleCallback>"u"?t=>setTimeout(t,1):requestIdleCallback;let K=!1,$=!1,lt=[],at=[];function jt(){K=!1;const t=lt.slice();lt=[],wt(t)}function Bt(){$=!1;const t=at.slice();at=[],wt(t)}function qr(t){K||(K=!0,queueMicrotask(jt)),lt.push(t)}function Pr(t){$||($=!0,In(Bt)),at.push(t)}function gn(){K&&jt(),$&&Bt()}const Ut=0,Rn=1;let B=!1,U=Ut,L=!1,M=null,x=!1,_t=!1;function Et(t){x=t}function yt(t){_t=t}let g=[],C=0;let o=null;function Z(t){o=t}let u=null;function z(t){u=t}let T=null;function On(t){T=t}let h=null,d=0,I=null;function Sn(t){I=t}let Vt=1,J=0,R=!1,f=null;function Gt(){return++Vt}function ct(){return!Q||f!==null&&f.l===null}function j(t){var p;var n=t.f;if(n&O)return!0;if(n&X){var r=t.deps,e=(n&D)!==0;if(r!==null){var s,l,a=(n&G)!==0,i=e&&u!==null&&!R,_=r.length;if(a||i){for(s=0;s<_;s++)l=r[s],(a||!((p=l==null?void 0:l.reactions)!=null&&p.includes(t)))&&(l.reactions??(l.reactions=[])).push(t);a&&(t.f^=G)}for(s=0;s<_;s++)if(l=r[s],j(l)&&Ct(l),l.wv>t.wv)return!0}(!e||u!==null&&!R)&&m(t,E)}return!1}function kn(t,n){for(var r=n;r!==null;){if(r.f&V)try{r.fn(t);return}catch{r.f^=V}r=r.parent}throw B=!1,t}function Dn(t){return(t.f&q)===0&&(t.parent===null||(t.parent.f&V)===0)}function tt(t,n,r,e){if(B){if(r===null&&(B=!1),Dn(n))throw t;return}r!==null&&(B=!0);{kn(t,n);return}}function Kt(t){var vt;var n=h,r=d,e=I,s=o,l=R,a=T,i=f,_=t.f;h=null,d=0,I=null,o=_&(A|W)?null:t,R=!x&&(_&D)!==0,T=null,f=t.ctx,J++;try{var p=(0,t.fn)(),c=t.deps;if(h!==null){var v;if(Y(t,d),c!==null&&d>0)for(c.length=d+h.length,v=0;v<h.length;v++)c[d+v]=h[v];else t.deps=c=h;if(!R)for(v=d;v<c.length;v++)((vt=c[v]).reactions??(vt.reactions=[])).push(t)}else c!==null&&d<c.length&&(Y(t,d),c.length=d);return s!==null&&J++,p}finally{h=n,d=r,I=e,o=s,R=l,T=a,f=i}}function Nn(t,n){let r=n.reactions;if(r!==null){var e=Wt.call(r,t);if(e!==-1){var s=r.length-1;s===0?r=n.reactions=null:(r[e]=r[s],r.pop())}}r===null&&n.f&y&&(h===null||!h.includes(n))&&(m(n,X),n.f&(D|G)||(n.f^=G),Y(n,0))}function Y(t,n){var r=t.deps;if(r!==null)for(var e=n;e<r.length;e++)Nn(t,r[e])}function nt(t){var n=t.f;if(!(n&q)){m(t,E);var r=u,e=f;u=t;try{n&ot?Tn(t):Lt(t),Ft(t),Pt(t);var s=Kt(t);t.teardown=typeof s=="function"?s:null,t.wv=Vt;var l=t.deps,a}catch(i){tt(i,t,r,e||t.ctx)}finally{u=r}}}function $t(){if(C>1e3){C=0;try{an()}catch(t){if(M!==null)tt(t,M,null);else throw t}}C++}function Zt(t){var n=t.length;if(n!==0){$t();var r=x;x=!0;try{for(var e=0;e<n;e++){var s=t[e];s.f&E||(s.f^=E);var l=[];zt(s,l),xn(l)}}finally{x=r}}}function xn(t){var n=t.length;if(n!==0)for(var r=0;r<n;r++){var e=t[r];if(!(e.f&(q|b)))try{j(e)&&(nt(e),e.deps===null&&e.first===null&&e.nodes_start===null&&(e.teardown===null?Mt(e):e.fn=null))}catch(s){tt(s,e,null,e.ctx)}}}function Cn(){if(L=!1,C>1001)return;const t=g;g=[],Zt(t),L||(C=0,M=null)}function rt(t){U===Ut&&(L||(L=!0,queueMicrotask(Cn))),M=t;for(var n=t;n.parent!==null;){n=n.parent;var r=n.f;if(r&(W|A)){if(!(r&E))return;n.f^=E}}g.push(n)}function zt(t,n){var r=t.first,e=[];t:for(;r!==null;){var s=r.f,l=(s&A)!==0,a=l&&(s&E)!==0,i=r.next;if(!a&&!(s&b))if(s&H){if(l)r.f^=E;else try{j(r)&&nt(r)}catch(v){tt(v,r,null,r.ctx)}var _=r.first;if(_!==null){r=_;continue}}else s&Tt&&e.push(r);if(i===null){let v=r.parent;for(;v!==null;){if(t===v)break t;var p=v.next;if(p!==null){r=p;continue t}v=v.parent}}r=i}for(var c=0;c<e.length;c++)_=e[c],n.push(_),zt(_,n)}function Jt(t){var n=U,r=g;try{$t();const s=[];U=Rn,g=s,L=!1,Zt(r);var e=t==null?void 0:t();return gn(),(g.length>0||s.length>0)&&Jt(),C=0,M=null,e}finally{U=n,g=r}}async function Fr(){await Promise.resolve(),Jt()}function Lr(t){var c;var n=t.f,r=(n&y)!==0;if(r&&n&q){var e=xt(t);return ft(t),e}if(o!==null){T!==null&&T.includes(t)&&un();var s=o.deps;t.rv<J&&(t.rv=J,h===null&&s!==null&&s[d]===t?d++:h===null?h=[t]:h.push(t),I!==null&&u!==null&&u.f&E&&!(u.f&A)&&I.includes(t)&&(m(u,O),rt(u)))}else if(r&&t.deps===null)for(var l=t,a=l.parent,i=l;a!==null;)if(a.f&y){var _=a;i=_,a=_.parent}else{var p=a;(c=p.deriveds)!=null&&c.includes(i)||(p.deriveds??(p.deriveds=[])).push(i);break}return r&&(l=t,j(l)&&Ct(l)),t.v}function Mr(t){const n=o;try{return o=null,t()}finally{o=n}}const bn=-7169;function m(t,n){t.f=t.f&bn|n}function Yr(t,n=!1,r){f={p:f,c:null,e:null,m:!1,s:t,x:null,l:null},Q&&!n&&(f.l={s:null,u:null,r1:[],r2:it(!1)})}function Hr(t){const n=f;if(n!==null){const a=n.e;if(a!==null){var r=u,e=o;n.e=null;try{for(var s=0;s<a.length;s++){var l=a[s];z(l.effect),Z(l.reaction),qt(l.fn)}}finally{z(r),Z(e)}}f=n.p,n.m=!0}return{}}function jr(t){if(!(typeof t!="object"||!t||t instanceof EventTarget)){if(ht in t)ut(t);else if(!Array.isArray(t))for(let n in t){const r=t[n];typeof r=="object"&&r&&ht in r&&ut(r)}}}function ut(t,n=new Set){if(typeof t=="object"&&t!==null&&!(t instanceof EventTarget)&&!n.has(t)){n.add(t),t instanceof Date&&t.getTime();for(let e in t)try{ut(t[e],n)}catch{}const r=Qt(t);if(r!==Object.prototype&&r!==Array.prototype&&r!==Map.prototype&&r!==Set.prototype&&r!==Date.prototype){const e=Xt(r);for(let s in e){const l=e[s].get;if(l)try{l.call(t)}catch{}}}}}export{st as $,Z as A,z as B,qn as C,o as D,u as E,et as F,wn as G,Qn as H,F as I,Er as J,_n as K,Tr as L,dr as M,w as N,b as O,br as P,Cr as Q,An as R,ht as S,Pn as T,Wn as U,pn as V,Xn as W,Yt as X,Rr as Y,mn as Z,k as _,Hr as a,cn as a0,tr as a1,pr as a2,it as a3,nr as a4,N as a5,At as a6,Hn as a7,Q as a8,Ar as a9,W as aA,rr as aB,er as aC,lr as aD,Un as aE,ar as aF,fr as aG,_r as aH,Pr as aI,Vn as aJ,Xt as aK,Jt as aL,Fr as aM,Gn as aN,fn as aa,Rt as ab,St as ac,Kn as ad,Nr as ae,rn as af,Ir as ag,ot as ah,tn as ai,ur as aj,Yn as ak,or as al,ir as am,Ln as an,Mn as ao,Zn as ap,pt as aq,cr as ar,zn as as,Qt as at,Sr as au,$n as av,Bn as aw,sr as ax,gt as ay,A as az,gr as b,mr as c,hn as d,hr as e,Or as f,Lr as g,kr as h,Mr as i,wt as j,f as k,jn as l,jr as m,wr as n,Jn as o,Yr as p,qt as q,yr as r,vr as s,xr as t,Dr as u,yn as v,qr as w,ct as x,S as y,Fn as z};
