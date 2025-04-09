import { useState, useEffect } from 'react';
import * as icons from '@qwikpage/icons';
import { renderIconDefinitionToSVGElement } from '@qwikpage/icons/es/helpers';

const iconsList: { [key: string]: any } = icons;
const QIcon = ({ name, style = {} }: { name: string; style?: any }) => {
    const [iconSvg, setIconSvg] = useState<string>('');

    useEffect(() => {
        const svgHTMLString = renderIconDefinitionToSVGElement(iconsList[name], {
            extraSVGAttrs: { width: '1em', height: '1em', fill: 'currentColor' },
        })
        setIconSvg(svgHTMLString);
    }, [name]);

    return iconSvg ? (
        <span
            className='anticon'
            style={{
                fontSize: '18px',
                verticalAlign: 'middle',
                ...style,
            }}
            dangerouslySetInnerHTML={{ __html: iconSvg }}
        />
    ) : null
};

export default QIcon;