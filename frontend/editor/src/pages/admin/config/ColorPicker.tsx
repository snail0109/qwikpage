import { ColorPicker } from 'antd';
/**
 * 
 * MColorPicker 默认值不显示,这边单独处理一下
 * @returns React.FC
 */
const ProjectColorPicker = (props: any) => {
  const { disabled } = props;
  const handleChange = (color: any) => {
    props.onChange(color.toHexString());
  };
  const handleClear = () => {
    props.onChange('');
  };
  return <ColorPicker disabled={disabled} format="hex" showText allowClear defaultValue={props.value} value={props.value} onChange={handleChange} onClear={handleClear} />;
};

export default ProjectColorPicker;
