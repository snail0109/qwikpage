import{r as o,j as r,B as I}from"./index-CrRWnIjq.js";import{F as n,x as d,y as F}from"./Page-kYXsiZfv.js";import{useFormContext as R}from"./context-PBsMsbgj.js";import{R as m}from"./index-BFS5rlf2.js";import{R as b}from"./MinusOutlined-BsLWaGMW.js";import"./index-BTSRI9HO.js";import"./index-BCFTpXN0.js";const y=({type:x,config:i,elements:t},c)=>{const[f,a]=o.useState(!0),{initValues:u}=R();return o.useEffect(()=>{u(x,i.props.formItem.name,[{}])},[]),o.useImperativeHandle(c,()=>({show(){a(!0)},hide(){a(!1)}})),f&&r.jsx("div",{style:i.style,children:r.jsx(n.List,{...i.props.formItem,children:(j,{add:e,remove:h})=>r.jsxs(r.Fragment,{children:[j.map(({key:p,name:s,...C})=>r.jsxs(d,{style:{display:"flex",width:"100%",marginBottom:8,padding:"0 10px"},align:"baseline",children:[r.jsx(d,{align:"baseline",direction:i.props.formItem.direction,children:t!=null&&t.length?t==null?void 0:t.map(l=>r.jsx(F,{item:{...l,name:s}},l.id)):null},p),r.jsx(m,{onClick:()=>e({title:"-",dataIndex:s+1},s+1)}),r.jsx(b,{onClick:()=>h(s)})]},p)),r.jsx(n.Item,{children:r.jsx(I,{type:"dashed",onClick:()=>e(),block:!0,icon:r.jsx(m,{}),children:"新增一条"})})]})})})},M=o.forwardRef(y);export{M as default};
//# sourceMappingURL=FormList-DuAi40ax.js.map
