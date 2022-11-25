//
//  File.swift
//  
//
//  Created by Shota Shimazu on 2022/11/26.
//

import SQLite


open calss SQLiteBackend {
    
    init() {
        do {
            let db = try Connection("xnu/internal/internalHostedStates.sqlite3")
            initializeTable()
        }
        
        catch {
            print("ERROR")
        }
    }

    func initializeTable() {
        let users = Table("core_overlay_internal_hosted_states")
        let id = Expression<Int64>("id")
        let key = Expression<String?>("key")
        let value = Expression<String>("value")

        try db.run(users.create { t in
            t.column(id, primaryKey: true)
            t.column(key, unique: true)
            t.column(value)
        })
    }
}
