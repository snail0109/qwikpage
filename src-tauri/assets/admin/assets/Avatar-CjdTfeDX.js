import{r,j as l}from"./index-CrRWnIjq.js";import{A as n}from"./Page-kYXsiZfv.js";import"./index-BFS5rlf2.js";import"./index-BTSRI9HO.js";import"./index-BCFTpXN0.js";const m=({config:e},i)=>{const[u,a]=r.useState(!0);r.useImperativeHandle(i,()=>({show(){a(!0)},hide(){a(!1)}}));const{textavatar:c,size:t,...s}=e.props,p=r.useMemo(()=>{if(!t)return"default";if(["large","small","default"].includes(t==null?void 0:t.toString()))return t;const o=Number(t.toString().replace("px",""));return isNaN(Number(o))?"default":Number(o)},[t]);return u&&l.jsx(n,{style:e.style,...s,src:s.src||void 0,size:p,children:e.props.textavatar})},N=r.forwardRef(m);export{N as default};
//# sourceMappingURL=Avatar-CjdTfeDX.js.map
