import { ComponentType } from '@materials/types';
import { useState, useEffect, useImperativeHandle, forwardRef, useCallback } from 'react';

interface LinkProps {
  href: string;
  target: '_self' | '_blank' | '_parent' | '_top';
  type: 'redirect' | 'download';
  text: string;
  [key: string]: any;
}

/**
 * 处理链接跳转
 * @param href 链接地址
 * @param target 打开方式
 * @param currentSearch 当前URL的查询参数
 */
const handleLinkJump = (href: string, target: string = '_self', currentSearch: string = '') => {
  if (!href) return;

  let url = href;

  // 处理非外部链接（非http/https/mailto/tel等）
  if (!/^(https?:|mailto:|tel:)/.test(href)) {
    const projectRootPath = window.location.pathname.split('/').slice(0, 3).join('/');

    if (href === '/') {
      // 根路径，保持原样
      url = projectRootPath;
      
    } else if (href.startsWith('/') && href.length > 1) {
      // 移除开头的斜杠
      url = href.substring(1);
    }
    // 其他情况（不以斜杠开头）保持原样

    // 如果是新窗口打开，需要携带当前查询参数
    if (target === '_blank' && currentSearch) {
      url += (url.includes('?') ? '&' : '?') + currentSearch;
    }
  }

  // 处理不同的打开方式
  switch (target) {
    case '_blank':
      window.open(url, '_blank');
      break;
    case '_parent':
      window.parent.location.href = url;
      break;
    case '_top':
      if (window.top) {
        window.top.location.href = url;
      }
      break;
    case '_self':
    default:
      window.location.href = url;
  }
};


/**
 *
 * @param props 组件本身属性
 * @param style 组件样式
 * @returns
 */
const MLink = ({ config }: ComponentType, ref: any) => {
  const [text, setText] = useState('');
  const [visible, setVisible] = useState(true);
  const props = config.props as LinkProps;

  useEffect(() => {
    const originText = config.props?.text?.toString() || '';
    setText(originText);
  }, [config.props.text]);

  const handleClick = useCallback((e: React.MouseEvent) => {
    if (!props.href) return;

    // 阻止默认行为，我们自己处理跳转
    e.preventDefault();

    // 如果是下载文件
    if (props.type === 'download') {
      const a = document.createElement('a');
      a.href = props.href;
      a.download = ''; // 空字符串表示使用默认文件名
      a.target = '_blank';
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      return;
    }

    // 获取当前URL的查询参数
    const currentSearch = window.location.search.slice(1);

    // 处理跳转
    handleLinkJump(props.href, props.target, currentSearch);
  }, [props.href, props.target, props.type]);


  // 对外暴露方法
  useImperativeHandle(ref, () => {
    return {
      show() {
        setVisible(true);
      },
      hide() {
        setVisible(false);
      },
    };
  });

  return (
    visible && (
      <a
        style={config.style}
        download={props.type}
        onClick={handleClick}
        {...props}>
        {text || "超链接文本占位"}
      </a>
    )
  );
};
export default forwardRef(MLink);
