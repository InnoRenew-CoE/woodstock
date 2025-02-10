import{a as p,t as y,c as ee}from"../chunks/disclose-version.DhCj0zca.js";import{p as O,a2 as ae,a as P,b as l,c as t,g as e,r as a,t as Q,d as z,s as H,a3 as E,e as x,a0 as te}from"../chunks/runtime.BNiZENzE.js";import{i as G,s as R,b as T,a as N}from"../chunks/props.ubv3QNbn.js";import{e as re,i as se}from"../chunks/each.BpeMN_eK.js";import{p as ie,s as oe}from"../chunks/stores.BrkdvXsF.js";import{M,t as A,s as U,b as C}from"../chunks/index.5w0jbpJt.js";import{b as J}from"../chunks/this.COEs-G5c.js";import"../chunks/legacy.C5_a2vTT.js";import{d as ne,p as de}from"../chunks/events.CG0BT_pH.js";import{b as le}from"../chunks/window.W74ZS_dW.js";import{n as ve}from"../chunks/notifications.Ce6HBQ0q.js";import{o as ce}from"../chunks/index-client.CO5LStFK.js";import{v as me}from"../chunks/index.BRh5ASk-.js";import{g as ge}from"../chunks/entry.J2Cw3Jae.js";var pe=y('<div class="bg-primary p-5 text-white"><div class="grid gap-2 items-center justify-center"><div id="partners" class="flex gap-3 h-[3rem] justify-center"><img src="../innorenew.svg"></div></div></div>');function fe(v){var o=pe();p(v,o)}var _e=(v,o)=>x(o,!e(o)),ue=y('<button><img src="../menu.svg" alt="menu" class="w-5"></button>'),be=(v,o)=>x(o,!1),he=y(`<div class="z-10 absolute bottom-0 translate-y-full bg-secondary text-white right-0 left-0 p-3
                sm:relative sm:flex sm:translate-y-0 sm:bg-transparent sm:text-primary"><ul class="pl-8 py-4 grid sm:flex gap-3 sm:gap-10"><li><a class="flex items-center gap-3 sm:hover:text-accent" href="/"><!>Home</a></li> <li><a class="flex items-center gap-3 sm:hover:text-accent" href="/account/login"><!>About</a></li> <li><a class="flex items-center gap-3 sm:hover:text-accent" href="/account/login"><!>Contact</a></li></ul></div>`),xe=y('<div><div class="flex"><img src="../woodstock.svg" class="h-14 rounded-full"></div> <!> <!></div>');function ye(v,o){O(o,!0);const[F,I]=R(),W=()=>T(ie,"$page",F);let f=H(0),n=z(()=>e(f)<640),c=H(!1);var m=ee(),_=ae(m);{var q=d=>{var g=xe(),w=l(t(g),2);{var k=s=>{var i=ue();i.__click=[_e,c],p(s,i)};G(w,s=>{e(n)&&s(k)})}var B=l(w,2);{var r=s=>{var i=he(),u=t(i);u.__click=[be,c];var b=t(u),$=t(b),j=t($),D=z(()=>e(n)?"bg-white":"hidden");M(j,{src:"../home.svg",get class(){return e(D)}}),E(),a($),a(b);var h=l(b,2),S=t(h),V=t(S),X=z(()=>e(n)?"bg-white":"hidden");M(V,{src:"../book-open.svg",get class(){return e(X)}}),E(),a(S),a(h);var K=l(h,2),L=t(K),Y=t(L),Z=z(()=>e(n)?"bg-white":"hidden");M(Y,{src:"../contact.svg",get class(){return e(Z)}}),E(),a(L),a(K),a(u),a(i),A(1,i,()=>C),A(2,i,()=>C,()=>({duration:100})),p(s,i)};G(B,s=>{(e(c)||!e(n))&&s(r)})}a(g),Q(()=>U(g,`relative bg-dark-background px-5 ${(e(n)?"py-2":"")??""} flex items-center justify-between border-b`)),p(d,g)};G(_,d=>{W().url.pathname.includes("/app")||d(q)})}le("innerWidth",d=>x(f,de(d))),p(v,m),P(),I()}ne(["click"]);var we=y('<div><div class="border border-secondary/10 bg-light-background rounded-lg px-4 py-2 flex items-center gap-3"><!> <div><div class="font-bold"> </div> <div class="font-light"> </div></div></div></div>'),ke=y('<div id="layout" class="bg-light-background text-sm min-h-[100vh] grid grid-rows-[auto_1fr_auto]"><div><!></div> <div class="flex-1 h-full relative"><!> <div class="absolute right-0 bottom-0 top-0 flex flex-col justify-end gap-5 p-10 pointer-events-none"></div></div> <div class="bg-orange-300"><!></div></div>');function Ee(v,o){O(o,!0);const[F,I]=R(),W=()=>T(ve,"$notificationsStore",F);let f=H(void 0),n=H(void 0),c=H(void 0);te(()=>{var r,s;(r=e(f))==null||r.clientHeight,(s=e(n))==null||s.clientHeight,e(c)}),ce(async()=>{const r=await me();console.log(r),r!==200&&await ge("/")});var m=ke(),_=t(m),q=t(_);ye(q,{}),a(_),J(_,r=>x(f,r),()=>e(f));var d=l(_,2),g=t(d);oe(g,()=>o.children);var w=l(g,2);re(w,5,W,se,(r,s)=>{var i=we(),u=t(i),b=t(u);M(b,{src:"../bell.svg",get class(){return`${e(s).class??""} size-5 bg-secondary`}});var $=l(b,2),j=t($),D=t(j,!0);a(j);var h=l(j,2),S=t(h,!0);a(h),a($),a(u),a(i),Q(()=>{U(i,`${e(s).class??""} p-2 min-w-[250px] bg-dark-background border border-secondary/80 rounded-xl shadow-lg shadow-secondary/30`),N(D,e(s).title),N(S,e(s).body)}),A(1,i,()=>C),A(2,i,()=>C),p(r,i)}),a(w),a(d);var k=l(d,2),B=t(k);fe(B),a(k),J(k,r=>x(n,r),()=>e(n)),a(m),J(m,r=>x(c,r),()=>e(c)),p(v,m),P(),I()}export{Ee as component};
