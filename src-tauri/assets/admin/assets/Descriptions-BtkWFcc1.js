import{r as p,j as n,B as d}from"./index-C0qVWane.js";import{u as w,h as B,Z as C,i as N,f as y,_ as f,T as h,I as F,$ as b,m as M}from"./Page-D5iqHLgS.js";import{T as m}from"./index-DPKgr_Rt.js";import"./index-i6Qw7kf9.js";import"./index-DfNzS1Q2.js";const Y=({id:j,type:k,config:s},D)=>{var u;const[S,i]=p.useState(!0),[l,t]=p.useState([]),g=w(e=>e.page.pageData.variableData);p.useEffect(()=>{v({})},[s.api,((u=s.api)==null?void 0:u.sourceType)=="variable"?g:""]);const v=e=>{B(s.api,e).then(r=>{(r==null?void 0:r.code)===0&&(Array.isArray(r.data)?t(r.data):t([r.data]))})};p.useImperativeHandle(D,()=>({show(){i(!0)},hide(){i(!1)}}));const c=(e,r)=>{const o=s.events.find(a=>a.eventName===e);M(o==null?void 0:o.actions,r)},A=(e,r,o)=>{if(N(e)?r.type==="money"?e=y(e,"currency"):r.type==="number"?e=y(e):r.type==="date1"?e=f(e,"YYYY-MM-DD"):r.type==="date2"&&(e=f(e)):typeof s.props.empty>"u"?e="-":s.props.empty&&(e=s.props.empty),r.render)try{e=new Function("text","record",`return (${r.render})(text,record);`)(e,o)}catch(a){console.error(`列[${r.title}]渲染失败`,a),e="解析异常"}if(r.type==="text"){const a=n.jsx(d,{type:"link",onClick:()=>c(r.eventName,o),children:e.toString()});return r.ellipsis&&r.copyable?n.jsx(m,{title:e,children:n.jsx(h.Paragraph,{copyable:!0,style:{marginBottom:0},children:r.clickable?a:e.toString()})}):r.ellipsis?n.jsx(m,{title:e,children:r.clickable?a:e.toString()}):r.copyable?n.jsx(h.Paragraph,{copyable:!0,children:r.clickable?a:e.toString()}):r.clickable?n.jsx(d,{type:"link",onClick:()=>c(r.eventName,o),children:e.toString()}):e.toString()}return r.type==="image"?n.jsx(F,{src:e,width:30}):r.type==="tag"?Array.isArray(e)?e.map(a=>n.jsx(b,{children:a},a)):typeof e=="string"||typeof e=="number"?n.jsx(b,{children:e}):e==null?void 0:e.toString():e},T=p.useMemo(()=>s.props.items.map(e=>{var a;const r=((a=l[0])==null?void 0:a[e.name])||"",o=A(r,e,l[0]||{});return{key:e.key,label:e.label,children:o,span:e.span}}),[s.props.items,l]);return S&&n.jsx(C,{...s.props,items:T,"data-id":j,"data-type":k,style:s.style})},H=p.forwardRef(Y);export{H as default};