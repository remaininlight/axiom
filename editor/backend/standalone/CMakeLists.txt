find_package(PortAudio)

if (NOT PORTAUDIO_FOUND)
    message(WARNING "PortAudio could not be found, the standalone backend won't have audio output support.")
    set(PORTAUDIO_INCLUDE_DIRS "")
    set(PORTAUDIO_LIBRARIES "")
else ()
    message(STATUS "Found PortAudio in ${PORTAUDIO_LIBRARIES}")
    add_definitions(-DPORTAUDIO)

    # statically linking PortAudio on Windows needs setupapi.lib
    if (WIN32 AND AXIOM_STATIC_LINK)
        set(PORTAUDIO_LIBRARIES ${PORTAUDIO_LIBRARIES} setupapi)
    endif ()
endif ()

set(AXIOM_STANDALONE_PROPERTIES)
if (CMAKE_BUILD_TYPE STREQUAL "Release")
    set(AXIOM_STANDALONE_PROPERTIES ${AXIOM_STANDALONE_PROPERTIES} WIN32 MACOSX_BUNDLE)
endif ()
include_directories(${PORTAUDIO_INCLUDE_DIRS})
add_executable(axiom_standalone ${AXIOM_STANDALONE_PROPERTIES} main.cpp PkgInfo ${ICON_PATH})
set_source_files_properties(${ICON_PATH} PROPERTIES MACOSX_PACKAGE_LOCATION Resources)
set_source_files_properties(PkgInfo PROPERTIES MACOSX_PACKAGE_LOCATION .)

set_target_properties(axiom_standalone PROPERTIES
    PREFIX ""
    OUTPUT_NAME "Axiom Standalone"
    
    MACOSX_BUNDLE TRUE
    MACOSX_BUNDLE_INFO_PLIST "${RES_DIR}/Info.plist.in"
    MACOSX_BUNDLE_INFO_STRING "Axiom Standalone 0.3.2"
    MACOSX_BUNDLE_NAME "Axiom Standalone"
    MACOSX_BUNDLE_PACKAGE_TYPE "APPL")

target_link_libraries(axiom_standalone ${AXIOM_LINK_FLAGS} axiom_editor ${PORTAUDIO_LIBRARIES})

install(TARGETS axiom_standalone DESTINATION .)
install(CODE "
    set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH})
    set(Qt5Core_DIR ${Qt5Core_DIR})
    set(QT_LIBRARY_DIR ${QT_LIBRARY_DIR})
    set(QT_BINARY_DIR ${QT_BINARY_DIR})
    set(QT_QTCORE_LIBRARY_RELEASE ${QT_QTCORE_LIBRARY_RELEASE})
    set(QT_QTCORE_LIBRARY_DEBUG ${QT_QTCORE_LIBRARY_DEBUG})

    include(DeployQt5)
    fixup_qt5_executable(\"\${CMAKE_INSTALL_PREFIX}/Axiom Standalone.app\" qcocoa \"\" \"\" \"\" \"\")
")
