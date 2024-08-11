import * as yup from 'yup';

import { signIn } from '@/actions/login';
import EscrowLink from '@/components/EscrowLink';
import usePageNavigationStore from '@/store/pageNavigationStore';
import { yupResolver } from '@hookform/resolvers/yup';
import { Button, Form, Input } from 'antd';
import { useRouter } from 'next/navigation';
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { FormItem } from 'react-hook-form-antd';

const loginSchema = yup
  .object({
    email: yup.string().email().required(),
    password: yup.string().required(),
  })
  .required();

const SignIn = () => {
  const router = useRouter();

  const { setIsNavigating } = usePageNavigationStore();
  const [isLoading, setIsLoading] = useState(false);
  const { control, handleSubmit } = useForm({
    resolver: yupResolver(loginSchema),
  });

  const onFinish = async () => {
    await handleSubmit(async (values) => {
      setIsLoading(true);
      try {
        const result = await signIn(values.email, values.password);

        console.log('sign in result', result);

        setIsNavigating(true);

        if (result.isSuccess) {
          router.push('/home');
        } else {
          router.push(`/login?message=${result.message}`);
          setIsNavigating(false);
          setIsLoading(false);
        }
      } catch (error) {
        console.log('error', error);
        router.push('/login?message=Could not authenticate user');
        setIsNavigating(false);
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

        <div className="flex items-center gap-4 self-end">
          <EscrowLink href="/resetpassword" text="Forgot password" />

          <Form.Item className="mb-0 block gap-4 text-right">
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
        </div>
      </Form>
    </>
  );
};

export default SignIn;
