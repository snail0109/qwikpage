import{r as s,j as f,R as i}from"./index-C0qVWane.js";import{i as j}from"./index-BMEox4u4.js";import{useFormContext as E}from"./context-dW7PTdPs.js";import{F as R,z as V,o as y}from"./Page-D5iqHLgS.js";import"./index-DPKgr_Rt.js";import"./index-DfNzS1Q2.js";import"./MinusOutlined-DBQl31A9.js";import"./RedoOutlined-B23EvFCt.js";import"./index-i6Qw7kf9.js";const F=({type:n,config:e,onChange:o,onBlur:t,onPressEnter:a},d)=>{const{initValues:I}=E(),[x,m]=s.useState(!0),[b,p]=s.useState();s.useEffect(()=>{var u;const r=(u=e.props.formItem)==null?void 0:u.name,h=e.props.defaultValue;I(n,r,h)},[e.props.defaultValue]),s.useEffect(()=>{typeof e.props.formWrap.disabled=="boolean"&&p(e.props.formWrap.disabled)},[e.props.formWrap.disabled]);const c=r=>{o&&o({[e.props.formItem.name]:r})},W=r=>{t==null||t({[e.props.formItem.name]:r})},v=r=>{a==null||a({[e.props.formItem.name]:r})};s.useImperativeHandle(d,()=>({show(){m(!0)},hide(){m(!1)},enable(){p(!1)},disable(){p(!0)}}));const l=j;return x&&f.jsx(R.Item,{...e.props.formItem,children:f.jsx(V,{...y(e.props.formWrap,["prefixIcons","suffixIcons"]),disabled:b,variant:e.props.formWrap.variant||void 0,style:e.style,prefix:e.props.formWrap.prefixIcons?i.createElement(l[e.props.formWrap.prefixIcons]):null,suffix:e.props.formWrap.suffixIcons?i.createElement(l[e.props.formWrap.suffixIcons]):null,onChange:r=>c(r.target.value),onBlur:r=>W(r.target.value),onPressEnter:r=>v(r.target.value)})})},k=s.forwardRef(F);export{k as default};