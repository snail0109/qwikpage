import{r,j as l}from"./index-C0qVWane.js";import{useFormContext as x}from"./context-dW7PTdPs.js";import{F as w,z as W}from"./Page-D5iqHLgS.js";import"./index-DPKgr_Rt.js";import"./index-i6Qw7kf9.js";import"./index-DfNzS1Q2.js";const j=({type:d,config:e,onChange:t,onBlur:a},u)=>{const{initValues:f}=x(),[i,o]=r.useState(!0),[n,p]=r.useState();r.useEffect(()=>{var m;const s=(m=e.props.formItem)==null?void 0:m.name,v=e.props.defaultValue;f(d,s,v)},[e.props.defaultValue]),r.useEffect(()=>{typeof e.props.formWrap.disabled=="boolean"&&p(e.props.formWrap.disabled)},[e.props.formWrap.disabled]);const b=s=>{t==null||t({[e.props.formItem.name]:s})},I=s=>{a==null||a({[e.props.formItem.name]:s})};return r.useImperativeHandle(u,()=>({show(){o(!0)},hide(){o(!1)},enable(){p(!1)},disable(){p(!0)}})),i&&l.jsx(w.Item,{...e.props.formItem,children:l.jsx(W.Password,{...e.props.formWrap,disabled:n,variant:e.props.formWrap.variant||void 0,style:e.style,onChange:s=>b(s.target.value),onBlur:s=>I(s.target.value)})})},c=r.forwardRef(j);export{c as default};