#pragma once

#include <cstdint>
#include <vector>

namespace PollingRate {
    std::vector<uint8_t> get_magic_packet(int rate);
}