import{r as t,j as l}from"./index-CrRWnIjq.js";const c=({config:e},a)=>{var r;const[i,s]=t.useState(!0);t.useImperativeHandle(a,()=>({show(){s(!0)},hide(){s(!1)}}));const n=t.useMemo(()=>{const{top:u}=e.props.clip||{top:"0px"};let p="100%";const o=(x=>Number(x.replace("px","")))(u);return o!=0&&(p=`calc(100% + ${-o}px)`),p},[e.props.clip]);return i&&l.jsx("div",{style:e.style,children:l.jsx("iframe",{style:{position:"absolute",top:((r=e.props.clip)==null?void 0:r.top)||0,left:0,width:"100%",height:n,border:"none"},...e.props})})},h=t.forwardRef(c);export{h as default};
//# sourceMappingURL=IFrame-f_pekC1k.js.map
