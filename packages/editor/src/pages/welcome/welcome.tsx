import { useLocation } from 'react-router-dom';
import style from './index.module.less';
function Welcome() {
  const location = useLocation();
  return (
    <div className={style.welcome}>
      <img src="/imgs/welcome.svg" alt="" />
      <div className={style.title}>欢迎使用 QwikPage 页面设计器</div>
      {location.pathname === '/' && (
        <div className={style.content}>
          <p>
            当前需要正确的项目地址才可以访问，如果没有创建项目，请先去Marsview低代码平台{' '}
          </p>
        </div>
      )}
    </div>
  );
}
export default Welcome;
