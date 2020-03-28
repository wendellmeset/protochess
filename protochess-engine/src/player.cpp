//
// Created by raytran on 3/24/20.
//

#include <iostream>
#include "player.h"

namespace protochess_engine {
    Player::Player(int playerNum) : playerNum(playerNum), name("Anon") {}

    Player::Player(int playerNum, std::string name) : playerNum(playerNum), name(name) {}

    std::string Player::getName() {
        return name;
    }

    void Player::setPieces(std::map<boost::uuids::uuid, std::shared_ptr<Piece>> pieceMap) {
        pieces = std::move(pieceMap);

        for (auto &x:pieces) {
            allPieces = boost::dynamic_bitset<>(x.second->getBitset());
            break;
        }

        for (auto &x:pieces) {
            allPieces |= boost::dynamic_bitset<>(x.second->getBitset());
        }
    }

    std::map<boost::uuids::uuid, std::shared_ptr<Piece>> &Player::getPieces() {
        return pieces;
    }

    std::map<char, MovementPattern> &Player::getMovementMap() {
        return movementMap;
    }

    void Player::setMovementMap(std::map<char, MovementPattern> map) {
        movementMap = map;
    }


    std::map<char, MovementPattern> &Player::getCaptureMap() {
        return captureMap;
    }

    void Player::setCaptureMap(std::map<char, MovementPattern> map) {
        captureMap = map;
    }

    boost::dynamic_bitset<> Player::getAllPiecesBitset() {
        return allPieces;
    }

    std::shared_ptr<Piece> Player::getPieceAt(Location loc) {
        for (auto &x:pieces) {
            if (x.second->getLocation() == loc) {
                return x.second;
            }
        }
        return nullptr;
    }

    void Player::update() {
        for (auto &x:pieces) {
            allPieces = boost::dynamic_bitset<>(x.second->getBitset());
            break;
        }

        for (auto &x:pieces) {
            allPieces |= boost::dynamic_bitset<>(x.second->getBitset());
        }
    }

    void Player::removePiece(boost::uuids::uuid id) {
        pieces.erase(id);
    }

    void Player::addPiece(std::shared_ptr<Piece> piece) {
        pieces.insert({piece->getId(), piece});
    }

    void Player::disableCastleRights() {
        canCastle_ = false;
    }

    bool Player::canCastle() {
        return canCastle_;
    }

    int Player::getPlayerNum() {
        return playerNum;
    }

    void Player::enableCastleRights() {
        canCastle_ = true;
    }

}
