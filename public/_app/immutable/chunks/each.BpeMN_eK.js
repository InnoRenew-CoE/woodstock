import{j as ae,k as re,E as Q,l as C,m as k,n as ne,g as z,H as ie,o as F,v as b,w as H,I as D,x as W,y as X,z as le,A as Z,q as fe,B as G,C as y,D as J,F as L,G as te,J as se,K as ue,L as ve,M as _e,N as de,O as oe,P as ce,Q as he,R as Ee,T as K,U as pe,V as Ae,W as Te}from"./runtime.BNiZENzE.js";function Ce(f,e){return e}function Ie(f,e,r,u){for(var v=[],_=e.length,t=0;t<_;t++)te(e[t].e,v,!0);var p=_>0&&v.length===0&&r!==null;if(p){var o=r.parentNode;se(o),o.append(r),u.clear(),g(f,e[0].prev,e[_-1].next)}ue(v,()=>{for(var T=0;T<_;T++){var d=e[T];p||(u.delete(d.k),g(f,d.prev,d.next)),ve(d.e,!p)}})}function we(f,e,r,u,v,_=null){var t=f,p={flags:e,items:new Map,first:null},o=(e&Q)!==0;if(o){var T=f;t=C?k(_e(T)):T.appendChild(ae())}C&&ne();var d=null,I=!1,m=de(()=>{var a=r();return pe(a)?a:a==null?[]:Z(a)});re(()=>{var a=z(m),i=a.length;if(I&&i===0)return;I=i===0;let s=!1;if(C){var w=t.data===ie;w!==(i===0)&&(t=F(),k(t),b(!1),s=!0)}if(C){for(var c=null,h,E=0;E<i;E++){if(H.nodeType===8&&H.data===oe){t=H,s=!0,b(!1);break}var A=a[E],n=u(A,E);h=$(H,p,c,null,A,n,E,v,e),p.items.set(n,h),c=h}i>0&&k(F())}if(!C){var l=ce;xe(a,p,t,v,e,(l.f&D)!==0,u)}_!==null&&(i===0?d?W(d):d=X(()=>_(t)):d!==null&&le(d,()=>{d=null})),s&&b(!0),z(m)}),C&&(t=H)}function xe(f,e,r,u,v,_,t,p){var q,V,B,U;var o=(v&he)!==0,T=(v&(y|L))!==0,d=f.length,I=e.items,m=e.first,a=m,i,s=null,w,c=[],h=[],E,A,n,l;if(o)for(l=0;l<d;l+=1)E=f[l],A=t(E,l),n=I.get(A),n!==void 0&&((q=n.a)==null||q.measure(),(w??(w=new Set)).add(n));for(l=0;l<d;l+=1){if(E=f[l],A=t(E,l),n=I.get(A),n===void 0){var j=a?a.e.nodes_start:r;s=$(j,e,s,s===null?e.first:s.next,E,A,l,u,v),I.set(A,s),c=[],h=[],a=s.next;continue}if(T&&ge(n,E,l,v),n.e.f&D&&(W(n.e),o&&((V=n.a)==null||V.unfix(),(w??(w=new Set)).delete(n))),n!==a){if(i!==void 0&&i.has(n)){if(c.length<h.length){var R=h[0],x;s=R.prev;var O=c[0],M=c[c.length-1];for(x=0;x<c.length;x+=1)P(c[x],R,r);for(x=0;x<h.length;x+=1)i.delete(h[x]);g(e,O.prev,M.next),g(e,s,O),g(e,M,R),a=R,s=M,l-=1,c=[],h=[]}else i.delete(n),P(n,a,r),g(e,n.prev,n.next),g(e,n,s===null?e.first:s.next),g(e,s,n),s=n;continue}for(c=[],h=[];a!==null&&a.k!==A;)(_||!(a.e.f&D))&&(i??(i=new Set)).add(a),h.push(a),a=a.next;if(a===null)continue;n=a}c.push(n),s=n,a=n.next}if(a!==null||i!==void 0){for(var N=i===void 0?[]:Z(i);a!==null;)(_||!(a.e.f&D))&&N.push(a),a=a.next;var S=N.length;if(S>0){var ee=v&Q&&d===0?r:null;if(o){for(l=0;l<S;l+=1)(B=N[l].a)==null||B.measure();for(l=0;l<S;l+=1)(U=N[l].a)==null||U.fix()}Ie(e,N,ee,I)}}o&&fe(()=>{var Y;if(w!==void 0)for(n of w)(Y=n.a)==null||Y.apply()}),G.first=e.first&&e.first.e,G.last=s&&s.e}function ge(f,e,r,u){u&y&&J(f.v,e),u&L?J(f.i,r):f.i=r}function $(f,e,r,u,v,_,t,p,o,T){var d=(o&y)!==0,I=(o&Ae)===0,m=d?I?Ee(v):K(v):v,a=o&L?K(t):t,i={i:a,v:m,k:_,a:null,e:null,prev:r,next:u};try{return i.e=X(()=>p(f,m,a),C),i.e.prev=r&&r.e,i.e.next=u&&u.e,r===null?e.first=i:(r.next=i,r.e.next=i.e),u!==null&&(u.prev=i,u.e.prev=i.e),i}finally{}}function P(f,e,r){for(var u=f.next?f.next.e.nodes_start:r,v=e?e.e.nodes_start:r,_=f.e.nodes_start;_!==u;){var t=Te(_);v.before(_),_=t}}function g(f,e,r){e===null?f.first=r:(e.next=r,e.e.next=r&&r.e),r!==null&&(r.prev=e,r.e.prev=e&&e.e)}export{we as e,Ce as i};
