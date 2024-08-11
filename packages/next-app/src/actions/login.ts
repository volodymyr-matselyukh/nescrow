'use server';

import { getErrorResponse } from '@/utils/supabase/processSupabaseErrors';
import { createClient } from '@/utils/supabase/serverClient';
import { headers } from 'next/headers';
import { redirect } from 'next/navigation';

export const signIn = async (email: string, password: string) => {
  const supabase = createClient();

  try {
    const { error } = await supabase.auth.signInWithPassword({
      email,
      password,
    });

    if (error?.status === 400) {
      return { isSuccess: false, message: "Invalid credentials" };
    }

    return getErrorResponse(error);
  } catch {
    return { isSuccess: false };
  }
};

export const signUp = async (email: string, password: string) => {
  const origin = headers().get('origin');
  const supabase = createClient();

  try {
    const { error } = await supabase.auth.signUp({
      email,
      password,
      options: {
        emailRedirectTo: `${origin}/auth/callback`,
      },
    });

    if (error) {
      console.log('Error during sign up', error);
      return redirect('/login?message=Could not register the user');
    }
  } catch {
    return redirect('/login?message=Could not register the user');
  }

  return redirect('/login?message=Check email to continue sign in process');
};
