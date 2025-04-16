import { useState, useEffect } from 'react';
import * as icons from '@qwikpage/icons';
import { renderIconDefinitionToSVGElement } from '@qwikpage/icons/es/helpers';

const iconsList: { [key: string]: any } = icons;

interface QIconProps {
  name: string;
  style?: React.CSSProperties;
  onClick?: () => void;
  className?: string;
}

const QIcon = ({ name, style = {}, onClick, className }: QIconProps) => {
  const [iconSvg, setIconSvg] = useState<string>('');

  useEffect(() => {
    const svgHTMLString = renderIconDefinitionToSVGElement(iconsList[name], {
      extraSVGAttrs: { width: '1em', height: '1em', fill: 'currentColor' },
    });
    setIconSvg(svgHTMLString);
  }, [name]);

  return iconSvg ? (
    <span
      className={`anticon ${className || ''}`}
      style={{
        fontSize: '18px',
        display: 'flex',
        ...style,
      }}
      onClick={onClick}
      dangerouslySetInnerHTML={{ __html: iconSvg }}
    />
  ) : null;
};

export default QIcon;