import{r as s,j as r}from"./index-CrRWnIjq.js";import{Y as h}from"./Page-kYXsiZfv.js";import"./index-BFS5rlf2.js";import"./index-BTSRI9HO.js";import"./index-BCFTpXN0.js";const m=({config:e,onAfterChange:a,onBeforeChange:i},o)=>{const[p,t]=s.useState(!0);return s.useImperativeHandle(o,()=>({show(){t(!0)},hide(){t(!1)}})),p&&r.jsx("div",{style:e.style,children:r.jsx(h,{...e.props,afterChange:a,beforeChange:i,children:e.props.imageUrls.map(l=>r.jsx("div",{children:r.jsx("img",{style:{height:`${e.props.height||160}`,width:`${e.props.width||"100%"}`},src:l.url})}))})})},C=s.forwardRef(m);export{C as default};
//# sourceMappingURL=Carousel-Ciri-rKj.js.map
