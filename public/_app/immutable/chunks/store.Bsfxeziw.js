import{F as Y,G as A,H as D,I as C,J as P,A as V,K as E,z as W,L as $,M as j,N as T,O as y,C as I,D as q,f as p,P as z,Q as B,R as F,S as G,U as J,V as K,y as Q,a as U,p as X,h as S,i as Z,k as x,c as N,l as ee,W as re,X as te,m as se,Y as ae}from"./runtime.C_0eRsjW.js";import{d as ne}from"./disclose-version.W2XoKDQc.js";const ie=new Set,L=new Set;function g(e){var O;var r=this,a=r.ownerDocument,s=e.type,u=((O=e.composedPath)==null?void 0:O.call(e))||[],t=u[0]||e.target,c=0,b=e.__root;if(b){var d=u.indexOf(b);if(d!==-1&&(r===document||r===window)){e.__root=r;return}var l=u.indexOf(r);if(l===-1)return;d<=l&&(c=d)}if(t=u[c]||e.target,t!==r){Y(e,"currentTarget",{configurable:!0,get(){return t||a}});var m=P,o=V;A(null),D(null);try{for(var n,i=[];t!==null;){var f=t.assignedSlot||t.parentNode||t.host||null;try{var _=t["__"+s];if(_!==void 0&&!t.disabled)if(C(_)){var[H,...M]=_;H.apply(t,[e,...M])}else _.call(t,e)}catch(v){n?i.push(v):n=v}if(e.cancelBubble||f===r||f===null)break;t=f}if(n){for(let v of i)queueMicrotask(()=>{throw v});throw n}}finally{e.__root=r,delete e.currentTarget,A(m),D(o)}}}const ue=["touchstart","touchmove"];function oe(e){return ue.includes(e)}function _e(e,r){var a=r==null?"":typeof r=="object"?r+"":r;a!==(e.__t??(e.__t=e.nodeValue))&&(e.__t=a,e.nodeValue=a==null?"":a+"")}function fe(e,r){return k(e,r)}function be(e,r){E(),r.intro=r.intro??!1;const a=r.target,s=S,u=p;try{for(var t=W(a);t&&(t.nodeType!==8||t.data!==$);)t=j(t);if(!t)throw T;y(!0),I(t),q();const c=k(e,{...r,anchor:t});if(p===null||p.nodeType!==8||p.data!==z)throw B(),T;return y(!1),c}catch(c){if(c===T)return r.recover===!1&&F(),E(),G(a),y(!1),fe(e,r);throw c}finally{y(s),I(u)}}const h=new Map;function k(e,{target:r,anchor:a,props:s={},events:u,context:t,intro:c=!0}){E();var b=new Set,d=o=>{for(var n=0;n<o.length;n++){var i=o[n];if(!b.has(i)){b.add(i);var f=oe(i);r.addEventListener(i,g,{passive:f});var _=h.get(i);_===void 0?(document.addEventListener(i,g,{passive:f}),h.set(i,1)):h.set(i,_+1)}}};d(J(ie)),L.add(d);var l=void 0,m=K(()=>{var o=a??r.appendChild(Q());return U(()=>{if(t){X({});var n=x;n.c=t}u&&(s.$$events=u),S&&ne(o,null),l=e(o,s)||{},S&&(V.nodes_end=p),t&&Z()}),()=>{var f;for(var n of b){r.removeEventListener(n,g);var i=h.get(n);--i===0?(document.removeEventListener(n,g),h.delete(n)):h.set(n,i)}L.delete(d),R.delete(l),o!==a&&((f=o.parentNode)==null||f.removeChild(o))}});return R.set(l,m),l}let R=new WeakMap;function he(e){const r=R.get(e);r&&r()}function ce(e,r,a){if(e==null)return r(void 0),N;const s=ee(()=>e.subscribe(r,a));return s.unsubscribe?()=>s.unsubscribe():s}let w=!1;function pe(e,r,a){const s=a[r]??(a[r]={store:null,source:te(void 0),unsubscribe:N});if(s.store!==e)if(s.unsubscribe(),s.store=e??null,e==null)s.source.v=void 0,s.unsubscribe=N;else{var u=!0;s.unsubscribe=ce(e,t=>{u?s.source.v=t:ae(s.source,t)}),u=!1}return se(s.source)}function ve(){const e={};return re(()=>{for(var r in e)e[r].unsubscribe()}),e}function ye(e){var r=w;try{return w=!1,[e(),w]}finally{w=r}}export{_e as a,pe as b,ye as c,be as h,fe as m,ve as s,he as u};
