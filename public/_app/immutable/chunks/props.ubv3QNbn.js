import{aj as L,M as ee,ak as ae,W as re,a5 as w,v as S,m as D,n as x,w as p,O as ne,a4 as te,al as se,J as ie,A as ue,am as fe,j as le,y as Y,p as _e,l as T,B as C,a as oe,_ as de,k as ce,Y as ve,H as he,o as be,x as U,z as $,aa as ge,R as J,Z as V,g as m,ad as Ee,ag as me,e as K,a9 as Se,an as pe,ao as Te,ap as ye,aq as Re,u as j,ar as Ie,as as Ae,af as Z,at as Ne,$ as Pe,au as Oe,av as we,S as Le,aw as De,d as z,N as Ye,ax as Ce}from"./runtime.BNiZENzE.js";import{b as Me,r as G,h as A,p as qe}from"./events.CG0BT_pH.js";import{b as Be}from"./disclose-version.DhCj0zca.js";import{s as Fe,g as He}from"./index.COyhmTID.js";const Ue=["touchstart","touchmove"];function $e(e){return Ue.includes(e)}let W=!0;function ke(e,a){var r=a==null?"":typeof a=="object"?a+"":a;r!==(e.__t??(e.__t=e.nodeValue))&&(e.__t=r,e.nodeValue=r==null?"":r+"")}function Ve(e,a){return Q(e,a)}function xe(e,a){L(),a.intro=a.intro??!1;const r=a.target,n=T,f=p;try{for(var t=ee(r);t&&(t.nodeType!==8||t.data!==ae);)t=re(t);if(!t)throw w;S(!0),D(t),x();const _=Q(e,{...a,anchor:t});if(p===null||p.nodeType!==8||p.data!==ne)throw te(),w;return S(!1),_}catch(_){if(_===w)return a.recover===!1&&se(),L(),ie(r),S(!1),Ve(e,a);throw _}finally{S(n),D(f)}}const E=new Map;function Q(e,{target:a,anchor:r,props:n={},events:f,context:t,intro:_=!0}){L();var v=new Set,c=i=>{for(var s=0;s<i.length;s++){var u=i[s];if(!v.has(u)){v.add(u);var d=$e(u);a.addEventListener(u,A,{passive:d});var y=E.get(u);y===void 0?(document.addEventListener(u,A,{passive:d}),E.set(u,1)):E.set(u,y+1)}}};c(ue(Me)),G.add(c);var o=void 0,h=fe(()=>{var i=r??a.appendChild(le());return Y(()=>{if(t){_e({});var s=de;s.c=t}f&&(n.$$events=f),T&&Be(i,null),W=_,o=e(i,n)||{},W=!0,T&&(C.nodes_end=p),t&&oe()}),()=>{var d;for(var s of v){a.removeEventListener(s,A);var u=E.get(s);--u===0?(document.removeEventListener(s,A),E.delete(s)):E.set(s,u)}G.delete(c),i!==r&&((d=i.parentNode)==null||d.removeChild(i))}});return M.set(o,h),o}let M=new WeakMap;function Je(e,a){const r=M.get(e);return r?(M.delete(e),r(a)):Promise.resolve()}function Ke(e,a,r=!1){T&&x();var n=e,f=null,t=null,_=ge,v=r?ve:0,c=!1;const o=(i,s=!0)=>{c=!0,h(s,i)},h=(i,s)=>{if(_===(_=i))return;let u=!1;if(T){const d=n.data===he;!!_===d&&(n=be(),D(n),S(!1),u=!0)}_?(f?U(f):s&&(f=Y(()=>s(n))),t&&$(t,()=>{t=null})):(t?U(t):s&&(t=Y(()=>s(n))),f&&$(f,()=>{f=null})),u&&S(!0)};ce(()=>{c=!1,a(o),c||h(null,null)},v),T&&(n=p)}let I=!1,q=Symbol();function Qe(e,a,r){const n=r[a]??(r[a]={store:null,source:J(void 0),unsubscribe:V});if(n.store!==e&&!(q in r))if(n.unsubscribe(),n.store=e??null,e==null)n.source.v=void 0,n.unsubscribe=V;else{var f=!0;n.unsubscribe=Fe(e,t=>{f?n.source.v=t:K(n.source,t)}),f=!1}return e&&q in r?He(e):m(n.source)}function Xe(e,a){return e.set(a),a}function ea(){const e={};function a(){Ee(()=>{for(var r in e)e[r].unsubscribe();me(e,q,{enumerable:!1,value:!0})})}return[e,a]}function aa(){I=!0}function je(e){var a=I;try{return I=!1,[e(),I]}finally{I=a}}function k(e){for(var a=C,r=C;a!==null&&!(a.f&(Ie|Ae));)a=a.parent;try{return Z(a),e()}finally{Z(r)}}function ra(e,a,r,n){var H;var f=(r&Ne)!==0,t=!Pe||(r&Oe)!==0,_=(r&we)!==0,v=(r&Ce)!==0,c=!1,o;_?[o,c]=je(()=>e[a]):o=e[a];var h=Le in e||De in e,i=_&&(((H=Se(e,a))==null?void 0:H.set)??(h&&a in e&&(l=>e[a]=l)))||void 0,s=n,u=!0,d=!1,y=()=>(d=!0,u&&(u=!1,v?s=j(n):s=n),s);o===void 0&&n!==void 0&&(i&&t&&pe(),o=y(),i&&i(o));var b;if(t)b=()=>{var l=e[a];return l===void 0?y():(u=!0,d=!1,l)};else{var B=k(()=>(f?z:Ye)(()=>e[a]));B.f|=Te,b=()=>{var l=m(B);return l!==void 0&&(s=void 0),l===void 0?s:l}}if(!(r&ye))return b;if(i){var X=e.$$legacy;return function(l,g){return arguments.length>0?((!t||!g||X||c)&&i(g?b():l),l):b()}}var N=!1,F=!1,P=J(o),R=k(()=>z(()=>{var l=b(),g=m(P);return N?(N=!1,F=!0,g):(F=!1,P.v=l)}));return f||(R.equals=Re),function(l,g){if(arguments.length>0){const O=g?m(R):t&&_?qe(l):l;return R.equals(O)||(N=!0,K(P,O),d&&s!==void 0&&(s=O),j(()=>m(R))),l}return m(R)}}export{ke as a,Qe as b,W as c,aa as d,Xe as e,xe as h,Ke as i,Ve as m,ra as p,ea as s,Je as u};
