import{s as o,g as c}from"./index.DEYB36my.js";import{a0 as b,n as f,g as l,a5 as _,a6 as d,e as p}from"./runtime.BWQzXbsj.js";let s=!1,i=Symbol();function v(e,n,u){const r=u[n]??(u[n]={store:null,source:b(void 0),unsubscribe:f});if(r.store!==e&&!(i in u))if(r.unsubscribe(),r.store=e??null,e==null)r.source.v=void 0,r.unsubscribe=f;else{var t=!0;r.unsubscribe=o(e,a=>{t?r.source.v=a:p(r.source,a)}),t=!1}return e&&i in u?c(e):l(r.source)}function y(e,n){return e.set(n),n}function N(){const e={};function n(){_(()=>{for(var u in e)e[u].unsubscribe();d(e,i,{enumerable:!1,value:!0})})}return[e,n]}function S(){s=!0}function U(e){var n=s;try{return s=!1,[e(),s]}finally{s=n}}export{v as a,y as b,U as c,S as m,N as s};
