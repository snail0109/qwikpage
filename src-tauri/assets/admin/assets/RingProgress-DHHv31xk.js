import{r as e,j as r,S as f}from"./index-CrRWnIjq.js";import{u as h,h as x,R}from"./Page-kYXsiZfv.js";import"./index-BFS5rlf2.js";import"./index-BTSRI9HO.js";import"./index-BCFTpXN0.js";const b=({config:a},l)=>{var p;const[u,d]=e.useState(0),[g,i]=e.useState(!0),[c,o]=e.useState(!0),m=h(t=>t.page.pageData.variableData);e.useEffect(()=>{n({})},[a.api,((p=a.api)==null?void 0:p.sourceType)=="variable"?m:""]);const n=t=>{i(!0),x(a.api,t).then(s=>{(s==null?void 0:s.code)===0&&(typeof s.data=="number"?d(s.data):console.error("[RingProgress]数据格式错误，进度条需要0-1的数字。")),i(!1)})};return e.useImperativeHandle(l,()=>({show(){o(!0)},hide(){o(!1)},update:t=>{n(t)}})),c&&r.jsx("div",{style:a.style,children:r.jsx(f,{spinning:g,size:"large",wrapperClassName:"spin-loading",children:r.jsx(R,{...a.props,percent:u})})})},y=e.forwardRef(b);export{y as default};
//# sourceMappingURL=RingProgress-DHHv31xk.js.map
