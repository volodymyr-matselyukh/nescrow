import { useRouterWrapper } from '@/hooks/useRouterWrapper';
import { createClient } from '@/utils/supabase/browserClient';
import { LogoutOutlined, UserOutlined } from '@ant-design/icons';
import { User } from '@supabase/supabase-js';
import { Dropdown, MenuProps, message } from 'antd';
import { useEffect, useState } from 'react';

const items: MenuProps['items'] = [
  {
    label: 'Sign out',
    key: '1',
    icon: <LogoutOutlined />,
  },
];

const LoggedInUser = () => {
  const { routerPush } = useRouterWrapper();
  const [user, setUser] = useState<User | null>();
  const supabase = createClient();

  useEffect(() => {
    supabase.auth
      .getUser()
      .then((userObject) => {
        setUser(userObject.data.user);
      })
      .catch((error) => {
        console.log("Couldn't get user", error);
      });
  }, []);

  const signOut = async () => {
    await supabase.auth.signOut();

    routerPush('/login');
  };

  const menuProps = {
    items,
    onClick: signOut,
  };

  const copyToClipBoard = async () => {
    if (user?.email) {
      await navigator.clipboard.writeText(user?.email);

      await message.info('Address copied to clipboard');
    }
  };

  return (
    <Dropdown.Button
      menu={menuProps}
      placement="bottomRight"
      icon={<UserOutlined />}
      className="w-max"
      onClick={copyToClipBoard}
    >
      {`User: ${user?.email ?? '--'}`}
    </Dropdown.Button>
  );
};

export default LoggedInUser;
