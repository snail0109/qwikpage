import{r as a,j as i,S as h}from"./index-CrRWnIjq.js";import{u as f,h as x,l as v}from"./Page-kYXsiZfv.js";import"./index-BFS5rlf2.js";import"./index-BTSRI9HO.js";import"./index-BCFTpXN0.js";const L=({config:e},d)=>{var l;const[u,r]=a.useState([]),[c,n]=a.useState(!0),[m,o]=a.useState(!0),y=f(t=>t.page.pageData.variableData);a.useEffect(()=>{p({})},[e.api,((l=e.api)==null?void 0:l.sourceType)=="variable"?y:""]);const p=t=>{n(!0),x(e.api,t).then(s=>{(s==null?void 0:s.code)===0&&(Array.isArray(s.data)?r(s.data):(console.error("[TinyLine]","data数据格式错误，请检查"),r([]))),n(!1)})};return a.useImperativeHandle(d,()=>({show(){o(!0)},hide(){o(!1)},update:t=>{p(t)}})),m&&i.jsx("div",{style:e.style,children:i.jsx(h,{spinning:c,size:"large",wrapperClassName:"spin-loading",children:i.jsx(v,{...e.props,data:u})})})},T=a.forwardRef(L);export{T as default};
//# sourceMappingURL=TinyLine-CMHDTOfr.js.map
