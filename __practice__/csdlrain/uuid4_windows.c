#include "uuid4_windows.h"

void generate_uuid_v4(GUID* uuid) {
  HCRYPTPROV hProvider = 0;
  BOOL result;

  result = CryptAcquireContext(&hProvider, NULL, NULL, PROV_RSA_FULL,
                               CRYPT_VERIFYCONTEXT | CRYPT_SILENT);

  if (!result) {
    fprintf(stderr, "CryptAcquireContext failed with error: %lu\n",
            GetLastError());
    return;
  }

  result = CryptGenRandom(hProvider, sizeof(GUID), (BYTE*)uuid);
  CryptReleaseContext(hProvider, 0);

  if (!result) {
    fprintf(stderr, "CryptGenRandom failed with error: %lu\n", GetLastError());
    return;
  }

  uuid->Data3 = (uuid->Data3 & 0x0FFF) | 0x4000;
  uuid->Data4[0] = (uuid->Data4[0] & 0x3F) | 0x80;
}
