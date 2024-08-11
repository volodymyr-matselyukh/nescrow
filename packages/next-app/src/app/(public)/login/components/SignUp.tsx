import * as yup from 'yup';

import { yupResolver } from '@hookform/resolvers/yup';
import { Button, Form, Input } from 'antd';
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { FormItem } from 'react-hook-form-antd';
import { signUp } from '@/actions/login';

const MIN_PASSWORD_LENGTH = 8;

const loginSchema = yup
  .object({
    email: yup.string().email().required(),
    password: yup.string().min(MIN_PASSWORD_LENGTH).required(),
    repeat_password: yup
      .string()
      .oneOf([yup.ref('password')], 'Passwords must match')
      .required(),
  })
  .required();

const SignUp = () => {
  const [isLoading, setIsLoading] = useState(false);

  const { control, handleSubmit } = useForm({
    resolver: yupResolver(loginSchema),
  });

  const onFinish = async () => {
    await handleSubmit(async (values) => {
      setIsLoading(true);
      try {
        await signUp(values.email, values.password);
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

        <FormItem
          className="block"
          control={control}
          name="repeat_password"
          label="Repeat password *"
          layout="vertical"
        >
          <Input.Password size="large" />
        </FormItem>

        <Form.Item className="mb-0 block text-right" layout="vertical">
          <Button
            type="primary"
            htmlType="submit"
            size="large"
            loading={isLoading}
            disabled={isLoading}
          >
            Sing up
          </Button>
        </Form.Item>
      </Form>
    </>
  );
};

export default SignUp;
