import {
  Outlet,
  createFileRoute,
  redirect,
} from '@tanstack/react-router'

import { Sidebar } from '@/components/ui/sidebar'

export const Route = createFileRoute('/_auth')({
  beforeLoad: ({ context, location }) => {
    if (!context.auth.isAuthenticated) {
      throw redirect({
        to: '/login',
        search: {
          redirect: location.href,
        },
      })
    }
  },
  component: AuthLayout,
})

function AuthLayout() {
  return (
    <div className="h-full">
      <Sidebar />
      <Outlet />
    </div>
  )
}
