#ifndef CSDLRAIN_UUID4_WINDOWS_H
#define CSDLRAIN_UUID4_WINDOWS_H

#include <rpc.h>
#include <stdio.h>
#include <wincrypt.h>
#include <windows.h>

void generate_uuid_v4(GUID* uuid);

#endif  // CSDLRAIN_UUID4_WINDOWS_H
