import{r as c,w as q,e as D,f as L,h as U,i as b,R as h,k as l,l as _,n as N,o as W,p as x,_ as F,q as G}from"./index-D_T4C72b.js";var S=c.createContext({});function H(n){return n.replace(/-(.)/g,function(e,o){return o.toUpperCase()})}function J(n,e){q(n,"[@ant-design/icons] ".concat(e))}function k(n){return b(n)==="object"&&typeof n.name=="string"&&typeof n.theme=="string"&&(b(n.icon)==="object"||typeof n.icon=="function")}function I(){var n=arguments.length>0&&arguments[0]!==void 0?arguments[0]:{};return Object.keys(n).reduce(function(e,o){var r=n[o];switch(o){case"class":e.className=r,delete e.class;break;default:delete e[o],e[H(o)]=r}return e},{})}function v(n,e,o){return o?h.createElement(n.tag,l(l({key:e},I(n.attrs)),o),(n.children||[]).map(function(r,a){return v(r,"".concat(e,"-").concat(n.tag,"-").concat(a))})):h.createElement(n.tag,l({key:e},I(n.attrs)),(n.children||[]).map(function(r,a){return v(r,"".concat(e,"-").concat(n.tag,"-").concat(a))}))}function R(n){return U(n)[0]}function E(n){return n?Array.isArray(n)?n:[n]:[]}var en={width:"1em",height:"1em",fill:"currentColor","aria-hidden":"true",focusable:"false"},K=`
.anticon {
  display: inline-flex;
  align-items: center;
  color: inherit;
  font-style: normal;
  line-height: 0;
  text-align: center;
  text-transform: none;
  vertical-align: -0.125em;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.anticon > * {
  line-height: 1;
}

.anticon svg {
  display: inline-block;
}

.anticon::before {
  display: none;
}

.anticon .anticon-icon {
  display: block;
}

.anticon[tabindex] {
  cursor: pointer;
}

.anticon-spin::before,
.anticon-spin {
  display: inline-block;
  -webkit-animation: loadingCircle 1s infinite linear;
  animation: loadingCircle 1s infinite linear;
}

@-webkit-keyframes loadingCircle {
  100% {
    -webkit-transform: rotate(360deg);
    transform: rotate(360deg);
  }
}

@keyframes loadingCircle {
  100% {
    -webkit-transform: rotate(360deg);
    transform: rotate(360deg);
  }
}
`,M=function(e){var o=c.useContext(S),r=o.csp,a=o.prefixCls,i=K;a&&(i=i.replace(/anticon/g,a)),c.useEffect(function(){var s=e.current,m=D(s);L(i,"@ant-design-icons",{prepend:!0,csp:r,attachTo:m})},[])},Q=["icon","className","onClick","style","primaryColor","secondaryColor"],f={primaryColor:"#333",secondaryColor:"#E6E6E6",calculated:!1};function V(n){var e=n.primaryColor,o=n.secondaryColor;f.primaryColor=e,f.secondaryColor=o||R(e),f.calculated=!!o}function X(){return l({},f)}var d=function(e){var o=e.icon,r=e.className,a=e.onClick,i=e.style,s=e.primaryColor,m=e.secondaryColor,y=_(e,Q),C=c.useRef(),u=f;if(s&&(u={primaryColor:s,secondaryColor:m||R(s)}),M(C),J(k(o),"icon should be icon definiton, but got ".concat(o)),!k(o))return null;var t=o;return t&&typeof t.icon=="function"&&(t=l(l({},t),{},{icon:t.icon(u.primaryColor,u.secondaryColor)})),v(t.icon,"svg-".concat(t.name),l(l({className:r,onClick:a,style:i,"data-icon":t.name,width:"1em",height:"1em",fill:"currentColor","aria-hidden":"true"},y),{},{ref:C}))};d.displayName="IconReact";d.getTwoToneColors=X;d.setTwoToneColors=V;function z(n){var e=E(n),o=N(e,2),r=o[0],a=o[1];return d.setTwoToneColors({primaryColor:r,secondaryColor:a})}function Y(){var n=d.getTwoToneColors();return n.calculated?[n.primaryColor,n.secondaryColor]:n.primaryColor}var Z=["className","icon","spin","rotate","tabIndex","onClick","twoToneColor"];z(G.primary);var T=c.forwardRef(function(n,e){var o=n.className,r=n.icon,a=n.spin,i=n.rotate,s=n.tabIndex,m=n.onClick,y=n.twoToneColor,C=_(n,Z),u=c.useContext(S),t=u.prefixCls,g=t===void 0?"anticon":t,j=u.rootClassName,A=W(j,g,x(x({},"".concat(g,"-").concat(r.name),!!r.name),"".concat(g,"-spin"),!!a||r.name==="loading"),o),p=s;p===void 0&&m&&(p=-1);var P=i?{msTransform:"rotate(".concat(i,"deg)"),transform:"rotate(".concat(i,"deg)")}:void 0,$=E(y),w=N($,2),B=w[0],O=w[1];return c.createElement("span",F({role:"img","aria-label":r.name},C,{ref:e,tabIndex:p,onClick:m,className:A}),c.createElement(d,{icon:r,primaryColor:B,secondaryColor:O,style:P}))});T.displayName="AntdIcon";T.getTwoToneColor=Y;T.setTwoToneColor=z;export{T as I,S as a,z as b,Y as g,en as s,M as u,J as w};
