import * as yup from 'yup';

import { signIn } from '@/actions/login';
import { yupResolver } from '@hookform/resolvers/yup';
import { Button, Form, Input } from 'antd';
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { FormItem } from 'react-hook-form-antd';
import usePageNavigationStore from '@/store/pageNavigationStore';

const loginSchema = yup
  .object({
    email: yup.string().email().required(),
    password: yup.string().required(),
  })
  .required();

const SignIn = () => {
  const { setIsNavigating } = usePageNavigationStore();
  const [isLoading, setIsLoading] = useState(false);
  const { control, handleSubmit } = useForm({
    resolver: yupResolver(loginSchema),
  });

  const onFinish = async () => {
    await handleSubmit(async (values) => {
      setIsLoading(true);
      try {
        await signIn(values.email, values.password);
        setIsNavigating(true);
      } finally {
        setIsLoading(false);
      }
    })();
  };

  const onFinishFailed = (errorInfo: any) => {
    console.log('submit failed', errorInfo);
  };

  return (
    <>
      <Form
        className="flex flex-col"
        name="basic"
        style={{ maxWidth: 600 }}
        initialValues={{ remember: true }}
        onFinish={onFinish}
        onFinishFailed={onFinishFailed}
        autoComplete="off"
        layout="vertical"
      >
        <FormItem
          control={control}
          name="email"
          label="Email *"
          layout="vertical"
        >
          <Input size="large" />
        </FormItem>

        <FormItem
          control={control}
          name="password"
          label="Password *"
          layout="vertical"
        >
          <Input.Password size="large" />
        </FormItem>

        <Form.Item className="mb-0 block text-right">
          <Button
            type="primary"
            htmlType="submit"
            size="large"
            loading={isLoading}
            disabled={isLoading}
          >
            Sign in
          </Button>
        </Form.Item>
      </Form>
    </>
  );
};

export default SignIn;
