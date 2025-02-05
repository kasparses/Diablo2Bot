#[derive(Clone, Copy, Debug)]
pub enum Route {
    Act1StartToPotionSeller,
    Act2StartToPotionSeller,
    Act3StartToPotionSeller,
    Act4StartToPotionSeller,
    Act5StartToPotionSeller,

    Act1StartToStash,
    Act2StartToStash,
    Act3StartToStash,
    Act4StartToStash,
    Act5StartToStash,

    Act1StartToDeckardCain,
    Act2StartToDeckardCain,
    Act3StartToDeckardCain,
    Act4StartToDeckardCain,
    Act5StartToDeckardCain,

    Act1StartToWaypoint1,
    Act1StartToWaypoint2,
    Act1StartToWaypoint3,
    Act1StartToWaypoint4,
    Act2StartToWaypoint,
    Act3StartToWaypoint,
    Act4StartToWaypoint,
    Act5StartToWaypoint,
    // Act1StartToCharsi,
    // Act2StartToFara,
    // Act3StartToHratli,
    // Act4StartToHalbu,
    // Act5StartToLarzuk,
}
