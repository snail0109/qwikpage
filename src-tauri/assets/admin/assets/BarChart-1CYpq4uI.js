import{r as e,j as s,S as x}from"./index-C0qVWane.js";import{u as v,h as S,B as b}from"./Page-D5iqHLgS.js";import"./index-DPKgr_Rt.js";import"./index-i6Qw7kf9.js";import"./index-DfNzS1Q2.js";const j=({config:a},n)=>{var d;const[u,o]=e.useState([]),[h,p]=e.useState(!0),[c,i]=e.useState(!0),m=v(t=>t.page.pageData.variableData);e.useEffect(()=>{l({})},[a.api,((d=a.api)==null?void 0:d.sourceType)=="variable"?m:""]);const l=t=>{p(!0),S(a.api,t).then(r=>{(r==null?void 0:r.code)===0&&(Array.isArray(r.data)?o(r.data):(console.error("[BarChart]","data数据格式错误，请检查"),o([]))),p(!1)})};return e.useImperativeHandle(n,()=>({show(){i(!0)},hide(){i(!1)},update:t=>{l(t)}})),c&&s.jsx("div",{style:a.style,children:s.jsx(x,{spinning:h,size:"large",wrapperClassName:"spin-loading",children:s.jsx(b,{...a.props,color:a.props.seriesField?a.props.color:a.props.color[0],data:u})})})},E=e.forwardRef(j);export{E as default};