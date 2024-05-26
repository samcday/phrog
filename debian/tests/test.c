#define GMOBILE_USE_UNSTABLE_API
#include <gmobile.h>

int main()
{
  g_autoptr (GmDeviceInfo) info = NULL;

  gm_device_tree_get_compatibles (NULL, NULL);
  info = gm_device_info_new ((const char *const []){"Purism,Librem5", NULL});
  g_assert (GM_IS_DEVICE_INFO (info));
  return 0;
}
