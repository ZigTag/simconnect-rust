fn main() {
    println!("cargo:rustc-link-search=simconnectsdk/libsrc/lib");
    println!("cargo:rustc-link-lib=static=SimConnect");

    let bindings = bindgen::Builder::default()
        .header("libsrc/include/SimConnect.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_type("SIMCONNECT_UNUSED")
        .whitelist_type("SIMCONNECT_OBJECT_ID_USER")
        .whitelist_type("SIMCONNECT_CAMERA_IGNORE_FIELD")
        .whitelist_type("SIMCONNECT_CLIENTDATA_MAX_SIZE")
        .whitelist_type("SIMCONNECT_GROUP_PRIORITY_HIGHEST")
        .whitelist_type("SIMCONNECT_GROUP_PRIORITY_HIGHEST_MASKABLE")
        .whitelist_type("SIMCONNECT_GROUP_PRIORITY_STANDARD")
        .whitelist_type("SIMCONNECT_GROUP_PRIORITY_DEFAULT")
        .whitelist_type("SIMCONNECT_GROUP_PRIORITY_LOWEST")
        .whitelist_type("SIMCONNECT_CLIENTDATATYPE_INT")
        .whitelist_type("SIMCONNECT_CLIENTDATATYPE_INT")
        .whitelist_type("SIMCONNECT_CLIENTDATATYPE_INT")
        .whitelist_type("SIMCONNECT_CLIENTDATATYPE_INT")
        .whitelist_type("SIMCONNECT_CLIENTDATATYPE_FLOAT")
        .whitelist_type("SIMCONNECT_CLIENTDATATYPE_FLOAT")
        .whitelist_type("SIMCONNECT_CLIENTDATAOFFSET_AUTO")
        .whitelist_type("SIMCONNECT_RECV_ID")
        .whitelist_type("SIMCONNECT_DATATYPE")
        .whitelist_type("SIMCONNECT_EXCEPTION")
        .whitelist_type("SIMCONNECT_SIMOBJECT_TYPE")
        .whitelist_type("SIMCONNECT_STATE")
        .whitelist_type("SIMCONNECT_PERIOD")
        .whitelist_type("SIMCONNECT_MISSION_END")
        .whitelist_type("SIMCONNECT_CLIENT_DATA_PERIOD")
        .whitelist_type("SIMCONNECT_TEXT_TYPE")
        .whitelist_type("SIMCONNECT_TEXT_RESULT")
        .whitelist_type("SIMCONNECT_WEATHER_MODE")
        .whitelist_type("SIMCONNECT_FACILITY_LIST_TYPE")

        .whitelist_type("SIMCONNECT_RECV")
        .whitelist_type("SIMCONNECT_RECV_EXCEPTION")
        .whitelist_type("SIMCONNECT_RECV_OPEN")
        .whitelist_type("SIMCONNECT_RECV_QUIT")
        .whitelist_type("SIMCONNECT_RECV_EVENT")
        .whitelist_type("SIMCONNECT_RECV_EVENT_FILENAME")
        .whitelist_type("SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE")
        .whitelist_type("SIMCONNECT_RECV_EVENT_FRAME")
        .whitelist_type("SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED")
        .whitelist_type("SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED")
        .whitelist_type("SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED")
        .whitelist_type("SIMCONNECT_RECV_EVENT_RACE_END")
        .whitelist_type("SIMCONNECT_RECV_EVENT_RACE_LAP")
        .whitelist_type("SIMCONNECT_RECV_SIMOBJECT_DATA")
        .whitelist_type("SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE")
        .whitelist_type("SIMCONNECT_RECV_CLIENT_DATA")
        .whitelist_type("SIMCONNECT_RECV_WEATHER_OBSERVATION")
        .whitelist_type("SIMCONNECT_RECV_CLOUD_STATE")
        .whitelist_type("SIMCONNECT_RECV_ASSIGNED_OBJECT_ID")
        .whitelist_type("SIMCONNECT_RECV_RESERVED_KEY")
        .whitelist_type("SIMCONNECT_RECV_SYSTEM_STATE")
        .whitelist_type("SIMCONNECT_RECV_CUSTOM_ACTION")
        .whitelist_type("SIMCONNECT_RECV_EVENT_WEATHER_MODE")
        .whitelist_type("SIMCONNECT_RECV_FACILITIES_LIST")
        .whitelist_type("SIMCONNECT_DATA_FACILITY_AIRPORT")
        .whitelist_type("SIMCONNECT_RECV_AIRPORT_LIST")
        .whitelist_type("SIMCONNECT_DATA_FACILITY_WAYPOINT")
        .whitelist_type("SIMCONNECT_RECV_WAYPOINT_LIST")
        .whitelist_type("SIMCONNECT_DATA_FACILITY_NDB")
        .whitelist_type("SIMCONNECT_RECV_NDB_LIST")
        .whitelist_type("SIMCONNECT_DATA_FACILITY_VOR")
        .whitelist_type("SIMCONNECT_RECV_VOR_LIST")
        .whitelist_type("SIMCONNECT_RECV_PICK")

        
        .whitelist_function("SimConnect_MapClientEventToSimEvent")
        .whitelist_function("SimConnect_TransmitClientEvent")
        .whitelist_function("SimConnect_SetSystemEventState")
        .whitelist_function("SimConnect_AddClientEventToNotificationGroup")
        .whitelist_function("SimConnect_RemoveClientEvent")
        .whitelist_function("SimConnect_SetNotificationGroupPriority")
        .whitelist_function("SimConnect_ClearNotificationGroup")
        .whitelist_function("SimConnect_RequestNotificationGroup")
        .whitelist_function("SimConnect_AddToDataDefinition")
        .whitelist_function("SimConnect_ClearDataDefinition")
        .whitelist_function("SimConnect_RequestDataOnSimObject")
        .whitelist_function("SimConnect_RequestDataOnSimObjectType")
        .whitelist_function("SimConnect_SetDataOnSimObject")
        .whitelist_function("SimConnect_MapInputEventToClientEvent")
        .whitelist_function("SimConnect_SetInputGroupPriority")
        .whitelist_function("SimConnect_RemoveInputEvent")
        .whitelist_function("SimConnect_ClearInputGroup")
        .whitelist_function("SimConnect_SetInputGroupState")
        .whitelist_function("SimConnect_RequestReservedKey")
        .whitelist_function("SimConnect_SubscribeToSystemEvent")
        .whitelist_function("SimConnect_UnsubscribeFromSystemEvent")
        .whitelist_function("SimConnect_WeatherRequestInterpolatedObservation")
        .whitelist_function("SimConnect_WeatherRequestObservationAtStation")
        .whitelist_function("SimConnect_WeatherRequestObservationAtNearestStation")
        .whitelist_function("SimConnect_WeatherCreateStation")
        .whitelist_function("SimConnect_WeatherRemoveStation")
        .whitelist_function("SimConnect_WeatherSetObservation")
        .whitelist_function("SimConnect_WeatherSetModeServer")
        .whitelist_function("SimConnect_WeatherSetModeTheme")
        .whitelist_function("SimConnect_WeatherSetModeGlobal")
        .whitelist_function("SimConnect_WeatherSetModeCustom")
        .whitelist_function("SimConnect_WeatherSetDynamicUpdateRate")
        .whitelist_function("SimConnect_WeatherRequestCloudState")
        .whitelist_function("SimConnect_WeatherCreateThermal")
        .whitelist_function("SimConnect_WeatherRemoveThermal")
        .whitelist_function("SimConnect_AICreateParkedATCAircraft")
        .whitelist_function("SimConnect_AICreateEnrouteATCAircraft")
        .whitelist_function("SimConnect_AICreateNonATCAircraft")
        .whitelist_function("SimConnect_AICreateSimulatedObject")
        .whitelist_function("SimConnect_AIReleaseControl")
        .whitelist_function("SimConnect_AIRemoveObject")
        .whitelist_function("SimConnect_AISetAircraftFlightPlan")
        .whitelist_function("SimConnect_ExecuteMissionAction")
        .whitelist_function("SimConnect_CompleteCustomMissionAction")
        .whitelist_function("SimConnect_Close")
        .whitelist_function("SimConnect_RetrieveString")
        .whitelist_function("SimConnect_GetLastSentPacketID")
        .whitelist_function("SimConnect_Open")
        .whitelist_function("SimConnect_CallDispatch")
        .whitelist_function("SimConnect_GetNextDispatch")
        .whitelist_function("SimConnect_RequestResponseTimes")
        .whitelist_function("SimConnect_InsertString")
        .whitelist_function("SimConnect_CameraSetRelative6DOF")
        .whitelist_function("SimConnect_MenuAddItem")
        .whitelist_function("SimConnect_MenuDeleteItem")
        .whitelist_function("SimConnect_MenuAddSubItem")
        .whitelist_function("SimConnect_MenuDeleteSubItem")
        .whitelist_function("SimConnect_RequestSystemState")
        .whitelist_function("SimConnect_SetSystemState")
        .whitelist_function("SimConnect_MapClientDataNameToID")
        .whitelist_function("SimConnect_CreateClientData")
        .whitelist_function("SimConnect_AddToClientDataDefinition")
        .whitelist_function("SimConnect_ClearClientDataDefinition")
        .whitelist_function("SimConnect_RequestClientData")
        .whitelist_function("SimConnect_SetClientData")
        .whitelist_function("SimConnect_FlightLoad")
        .whitelist_function("SimConnect_FlightSave")
        .whitelist_function("SimConnect_FlightPlanLoad")
        .whitelist_function("SimConnect_Text")
        .whitelist_function("SimConnect_SubscribeToFacilities")
        .whitelist_function("SimConnect_UnsubscribeToFacilities")
        .whitelist_function("SimConnect_RequestFacilitiesList")
        
        .whitelist_type("SIMCONNECT_DATA_RACE_RESULT")
        .whitelist_type("SIMCONNECT_DATA_INITPOSITION")
        .whitelist_type("SIMCONNECT_DATA_MARKERSTATE")
        .whitelist_type("SIMCONNECT_DATA_WAYPOINT")
        .whitelist_type("SIMCONNECT_DATA_LATLONALT")
        .whitelist_type("SIMCONNECT_DATA_XYZ")


        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");
}