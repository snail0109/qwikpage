import Icon from '@ant-design/icons';
import type { CustomIconComponentProps } from '@ant-design/icons/lib/components/Icon';

const PageSvg = () => (
    <svg width="16px" height="16px" viewBox="0 0 16 16" version="1.1">
        <defs>
            <linearGradient x1="50%" y1="0%" x2="13.2713029%" y2="100%" id="linearGradient-1">
                <stop stopColor="#216EF7" offset="0%"></stop>
                <stop stopColor="#AB6DFF" offset="100%"></stop>
            </linearGradient>
        </defs>
        <g stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
            <path d="M9.81560689,0.00130601584 L9.81560689,3.00644845 C9.81560689,4.09697168 10.6997796,4.97983838 11.7889968,4.97983838 L14.7131663,4.97983838 L14.7131663,14.0004898 C14.7131663,15.1040731 13.7362664,16 12.6627214,16 L2.94335156,16 C1.87111256,16 1,15.1040731 1,14.0004898 L1,2.00081626 C1,0.897232879 1.86980655,0.00130601584 2.94335156,0.00130601584 L9.81560689,0.00130601584 L9.81560689,0.00130601584 Z M11.1216227,0.00130601584 L11.389356,0.00130601584 L14.7131663,3.52101869 L14.7131663,3.67382255 L11.7889968,3.67382255 C11.4209257,3.67382255 11.122343,3.37582487 11.1216227,3.00775447 L11.1216227,0 L11.1216227,0.00130601584 Z" fill="url(#linearGradient-1)" fillRule="nonzero"></path>
        </g>
    </svg>
);

const PageIcon = (props: Partial<CustomIconComponentProps>) => (
    <Icon component={PageSvg} {...props} />
);

export default PageIcon; 