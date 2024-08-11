import { AuthError } from '@supabase/supabase-js';

export const getErrorResponse = (error: AuthError | null | undefined) => {
  if (!error) {
    return { isSuccess: true };
  }

  console.log('Auth error', error);

  if (error.code === 'over_email_send_rate_limit') {
    return {
      isSuccess: false,
      message: 'Email send rate limit reached. Try later.',
    };
  }

  return {
    isSuccess: false,
    message: 'Auth error happened. Try later.',
  };
};
