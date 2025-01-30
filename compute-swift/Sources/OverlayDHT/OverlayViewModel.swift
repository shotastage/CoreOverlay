//
//  OverlayViewModel.swift
//  
//
//  Created by Shota Shimazu on 2023/01/13.
//

import Combine


class OverlayViewModel {
    let mainButtonPressed = PassthroughSubject<Void, Never>()
    private(set) var mainButtonIsOn = CurrentValueSubject<Bool, Never>(true)
    var subscriptions = Set<AnyCancellable>()

    init() {
        mutate()
    }

    func mutate() {
        mainButtonPressed.sink { [weak self] _ in
            guard let self = self else { return }
            self.mainButtonIsOn.send(!self.mainButtonIsOn.value)
        }.store(in: &subscriptions)
    }
}
