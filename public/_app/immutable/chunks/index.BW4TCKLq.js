import{O as va,l as da,ap as ra,k as C,F as L,G as ca,g as K,T as _a,U as X,D as H,v as N,aq as R,V as ia,m as ta,W as pa,M as na,q as sa,P as O,ar as B,as as J,at as U,au as ha,L as ma,av as ya,o as Ea,A as wa,ab as ga,I as Ta,am as fa,aw as $a,Y as ba,ag as Q,ak as Aa,ax as Ia,B as Ca,E as xa,ay as Na,az as Fa,h as ka,u as Sa,aA as Ra,aB as Oa,n as F,aC as Ma,aD as Da,al as Z,a6 as j}from"./runtime.DJIhzSjh.js";import{a as La}from"./render.BYfnFhVg.js";function Xa(e,a){return a}function Ha(e,a,r,t){for(var f=[],n=a.length,u=0;u<n;u++)ha(a[u].e,f,!0);var y=n>0&&f.length===0&&r!==null;if(y){var E=r.parentNode;ma(E),E.append(r),t.clear(),A(e,a[0].prev,a[n-1].next)}ya(f,()=>{for(var p=0;p<n;p++){var s=a[p];y||(t.delete(s.k),A(e,s.prev,s.next)),Ea(s.e,!y)}})}function Ja(e,a,r,t,f,n=null){var u=e,y={flags:a,items:new Map,first:null},E=(a&ra)!==0;if(E){var p=e;u=C?L(wa(p)):p.appendChild(va())}C&&ca();var s=null,g=!1,h=ga(()=>{var i=r();return Aa(i)?i:i==null?[]:na(i)});da(()=>{var i=K(h),o=i.length;if(g&&o===0)return;g=o===0;let l=!1;if(C){var w=u.data===_a;w!==(o===0)&&(u=X(),L(u),H(!1),l=!0)}if(C){for(var v=null,m,c=0;c<o;c++){if(N.nodeType===8&&N.data===Ta){u=N,l=!0,H(!1);break}var T=i[c],d=t(T,c);m=oa(N,y,v,null,T,d,c,f,a),y.items.set(d,m),v=m}o>0&&L(X())}if(!C){var _=fa;qa(i,y,u,f,a,(_.f&R)!==0,t)}n!==null&&(o===0?s?ia(s):s=ta(()=>n(u)):s!==null&&pa(s,()=>{s=null})),l&&H(!0),K(h)}),C&&(u=N)}function qa(e,a,r,t,f,n,u,y){var W,P,V,Y;var E=(f&$a)!==0,p=(f&(B|U))!==0,s=e.length,g=a.items,h=a.first,i=h,o,l=null,w,v=[],m=[],c,T,d,_;if(E)for(_=0;_<s;_+=1)c=e[_],T=u(c,_),d=g.get(T),d!==void 0&&((W=d.a)==null||W.measure(),(w??(w=new Set)).add(d));for(_=0;_<s;_+=1){if(c=e[_],T=u(c,_),d=g.get(T),d===void 0){var k=i?i.e.nodes_start:r;l=oa(k,a,l,l===null?a.first:l.next,c,T,_,t,f),g.set(T,l),v=[],m=[],i=l.next;continue}if(p&&Ba(d,c,_,f),d.e.f&R&&(ia(d.e),E&&((P=d.a)==null||P.unfix(),(w??(w=new Set)).delete(d))),d!==i){if(o!==void 0&&o.has(d)){if(v.length<m.length){var I=m[0],$;l=I.prev;var z=v[0],M=v[v.length-1];for($=0;$<v.length;$+=1)aa(v[$],I,r);for($=0;$<m.length;$+=1)o.delete(m[$]);A(a,z.prev,M.next),A(a,l,z),A(a,M,I),i=I,l=M,_-=1,v=[],m=[]}else o.delete(d),aa(d,i,r),A(a,d.prev,d.next),A(a,d,l===null?a.first:l.next),A(a,l,d),l=d;continue}for(v=[],m=[];i!==null&&i.k!==T;)(n||!(i.e.f&R))&&(o??(o=new Set)).add(i),m.push(i),i=i.next;if(i===null)continue;d=i}v.push(d),l=d,i=d.next}if(i!==null||o!==void 0){for(var x=o===void 0?[]:na(o);i!==null;)(n||!(i.e.f&R))&&x.push(i),i=i.next;var D=x.length;if(D>0){var ua=f&ra&&s===0?r:null;if(E){for(_=0;_<D;_+=1)(V=x[_].a)==null||V.measure();for(_=0;_<D;_+=1)(Y=x[_].a)==null||Y.fix()}Ha(a,x,ua,g)}}E&&sa(()=>{var G;if(w!==void 0)for(d of w)(G=d.a)==null||G.apply()}),O.first=a.first&&a.first.e,O.last=l&&l.e}function Ba(e,a,r,t){t&B&&J(e.v,a),t&U?J(e.i,r):e.i=r}function oa(e,a,r,t,f,n,u,y,E,p){var s=(E&B)!==0,g=(E&Ia)===0,h=s?g?ba(f):Q(f):f,i=E&U?Q(u):u,o={i,v:h,k:n,a:null,e:null,prev:r,next:t};try{return o.e=ta(()=>y(e,h,i),C),o.e.prev=r&&r.e,o.e.next=t&&t.e,r===null?a.first=o:(r.next=o,r.e.next=o.e),t!==null&&(t.prev=o,t.e.prev=o.e),o}finally{}}function aa(e,a,r){for(var t=e.next?e.next.e.nodes_start:r,f=a?a.e.nodes_start:r,n=e.e.nodes_start;n!==t;){var u=Ca(n);f.before(n),n=u}}function A(e,a,r){a===null?e.first=r:(a.next=r,a.e.next=r&&r.e),r!==null&&(r.prev=a,r.e.prev=a&&a.e)}const Ua=()=>performance.now(),b={tick:e=>requestAnimationFrame(e),now:()=>Ua(),tasks:new Set};function la(){const e=b.now();b.tasks.forEach(a=>{a.c(e)||(b.tasks.delete(a),a.f())}),b.tasks.size!==0&&b.tick(la)}function za(e){let a;return b.tasks.size===0&&b.tick(la),{promise:new Promise(r=>{b.tasks.add(a={c:e,f:r})}),abort(){b.tasks.delete(a)}}}function S(e,a){e.dispatchEvent(new CustomEvent(a))}function Wa(e){if(e==="float")return"cssFloat";if(e==="offset")return"cssOffset";if(e.startsWith("--"))return e;const a=e.split("-");return a.length===1?a[0]:a[0]+a.slice(1).map(r=>r[0].toUpperCase()+r.slice(1)).join("")}function ea(e){const a={},r=e.split(";");for(const t of r){const[f,n]=t.split(":");if(!f||n===void 0)break;const u=Wa(f.trim());a[u]=n.trim()}return a}const Pa=e=>e;function Qa(e,a,r,t){var f=(e&Ra)!==0,n=(e&Ma)!==0,u=f&&n,y=(e&Da)!==0,E=u?"both":f?"in":"out",p,s=a.inert,g=a.style.overflow,h,i;function o(){var c=fa,T=O;Z(null),j(null);try{return p??(p=r()(a,(t==null?void 0:t())??{},{direction:E}))}finally{Z(c),j(T)}}var l={is_global:y,in(){var c;if(a.inert=s,!f){i==null||i.abort(),(c=i==null?void 0:i.reset)==null||c.call(i);return}n||h==null||h.abort(),S(a,"introstart"),h=q(a,o(),i,1,()=>{S(a,"introend"),h==null||h.abort(),h=p=void 0,a.style.overflow=g})},out(c){if(!n){c==null||c(),p=void 0;return}a.inert=!0,S(a,"outrostart"),i=q(a,o(),h,0,()=>{S(a,"outroend"),c==null||c()})},stop:()=>{h==null||h.abort(),i==null||i.abort()}},w=O;if((w.transitions??(w.transitions=[])).push(l),f&&La){var v=y;if(!v){for(var m=w.parent;m&&m.f&xa;)for(;(m=m.parent)&&!(m.f&Na););v=!m||(m.f&Fa)!==0}v&&ka(()=>{Sa(()=>l.in())})}}function q(e,a,r,t,f){var n=t===1;if(Oa(a)){var u,y=!1;return sa(()=>{if(!y){var w=a({direction:n?"in":"out"});u=q(e,w,r,t,f)}}),{abort:()=>{y=!0,u==null||u.abort()},deactivate:()=>u.deactivate(),reset:()=>u.reset(),t:()=>u.t()}}if(r==null||r.deactivate(),!(a!=null&&a.duration))return f(),{abort:F,deactivate:F,reset:F,t:()=>t};const{delay:E=0,css:p,tick:s,easing:g=Pa}=a;var h=[];if(n&&r===void 0&&(s&&s(0,1),p)){var i=ea(p(0,1));h.push(i,i)}var o=()=>1-t,l=e.animate(h,{duration:E});return l.onfinish=()=>{var w=(r==null?void 0:r.t())??1-t;r==null||r.abort();var v=t-w,m=a.duration*Math.abs(v),c=[];if(m>0){var T=!1;if(p)for(var d=Math.ceil(m/16.666666666666668),_=0;_<=d;_+=1){var k=w+v*g(_/d),I=ea(p(k,1-k));c.push(I),T||(T=I.overflow==="hidden")}T&&(e.style.overflow="hidden"),o=()=>{var $=l.currentTime;return w+v*g($/m)},s&&za(()=>{if(l.playState!=="running")return!1;var $=o();return s($,1-$),!0})}l=e.animate(c,{duration:m,fill:"forwards"}),l.onfinish=()=>{o=()=>t,s==null||s(t,1-t),f()}},{abort:()=>{l&&(l.cancel(),l.effect=null,l.onfinish=F)},deactivate:()=>{f=F},reset:()=>{t===0&&(s==null||s(1,0))},t:()=>o()}}const Va=e=>e;function Ya(e){const a=e-1;return a*a*a+1}function Za(e,{delay:a=0,duration:r=400,easing:t=Va}={}){const f=+getComputedStyle(e).opacity;return{delay:a,duration:r,easing:t,css:n=>`opacity: ${n*f}`}}function ja(e,{delay:a=0,duration:r=400,easing:t=Ya,axis:f="y"}={}){const n=getComputedStyle(e),u=+n.opacity,y=f==="y"?"height":"width",E=parseFloat(n[y]),p=f==="y"?["top","bottom"]:["left","right"],s=p.map(v=>`${v[0].toUpperCase()}${v.slice(1)}`),g=parseFloat(n[`padding${s[0]}`]),h=parseFloat(n[`padding${s[1]}`]),i=parseFloat(n[`margin${s[0]}`]),o=parseFloat(n[`margin${s[1]}`]),l=parseFloat(n[`border${s[0]}Width`]),w=parseFloat(n[`border${s[1]}Width`]);return{delay:a,duration:r,easing:t,css:v=>`overflow: hidden;opacity: ${Math.min(v*20,1)*u};${y}: ${v*E}px;padding-${p[0]}: ${v*g}px;padding-${p[1]}: ${v*h}px;margin-${p[0]}: ${v*i}px;margin-${p[1]}: ${v*o}px;border-${p[0]}-width: ${v*l}px;border-${p[1]}-width: ${v*w}px;min-${y}: 0`}}export{Ja as e,Za as f,Xa as i,ja as s,Qa as t};
